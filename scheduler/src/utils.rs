use crate::parser::QueryFragment;

use crate::scheduler;
use crate::scheduler::SCHEDULER_INSTANCE;
use crate::scheduler_interface::QueryInfo;
use datafusion::arrow::datatypes;
use datafusion::config::TableParquetOptions;
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::physical_plan::{FileScanConfig, ParquetExec};
use datafusion::execution::{object_store, RecordBatchStream};
use datafusion::parquet::{arrow, basic::Compression, file::properties::WriterProperties};
use datafusion::physical_plan::{self, ExecutionPlan, Statistics};
use datafusion::prelude::*;
use futures::stream::TryStreamExt;
use std::{error, path::Path, pin::Pin, sync::Arc};

/// Spills records obtained by polling `rb_stream` into `filename`.
pub async fn spill_records_to_disk(
    filename: &str,
    mut rb_stream: Pin<Box<dyn RecordBatchStream + Send>>,
    schema: datatypes::SchemaRef,
    num_rows_per_file: usize,
    filegroup_size: usize,
    _print: bool,
) -> Result<Vec<Vec<PartitionedFile>>, Box<dyn error::Error>> {
    // TODO(George): think about splitting large parquet files
    let mut partition_number = 0usize;
    let file_name = format!("{filename}_{partition_number}.parquet");
    let path_name = Path::new(file_name.as_str());
    let prefix = path_name.parent().unwrap();
    tokio::fs::create_dir_all(prefix).await?;
    let mut file_pq = tokio::fs::File::create(Path::new(
        format!("{filename}_{partition_number}.parquet").as_str(),
    ))
    .await?;

    // WriterProperties can be used to set Parquet file options
    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    let mut parquet_files_written: Vec<Vec<PartitionedFile>> =
        vec![Vec::<PartitionedFile>::new(); filegroup_size];
    let mut filenames = Vec::<PartitionedFile>::new();

    let mut writer =
        arrow::AsyncArrowWriter::try_new(file_pq, schema.clone(), Some(props.clone())).unwrap();
    let mut num_rows = 0;

    // write to disk
    while let Some(rec_batch) = rb_stream.try_next().await.unwrap() {
        if num_rows >= num_rows_per_file {
            writer.close().await?;
            filenames.push(
                PartitionedFile::from_path(format!("{filename}_{partition_number}.parquet"))
                    .expect("Parquet file name path error"),
            );
            parquet_files_written[partition_number % filegroup_size].push(
                PartitionedFile::from_path(format!("{filename}_{partition_number}.parquet"))
                    .expect("Parquet file name path error"),
            );

            partition_number += 1;
            num_rows = 0;
            file_pq = tokio::fs::File::create(Path::new(
                format!("{filename}_{partition_number}.parquet").as_str(),
            ))
            .await?;
            writer = arrow::AsyncArrowWriter::try_new(file_pq, schema.clone(), Some(props.clone()))
                .unwrap();
        }
        writer.write(&rec_batch).await.expect("Writing batch");
        num_rows += rec_batch.num_rows();
    }

    writer.close().await?;
    parquet_files_written[partition_number % filegroup_size].push(
        PartitionedFile::from_path(format!("{filename}_{partition_number}.parquet"))
            .expect("Parquet file name path error"),
    );

    partition_number += 1;

    Ok(parquet_files_written)
}

/// Create a [ParquetExec] physical plan node that scans Parquet partitions from `file_config`.
pub fn scan_from_parquet(file_config: FileScanConfig) -> Arc<dyn ExecutionPlan> {
    Arc::new(ParquetExec::new(
        file_config,
        None,
        None,
        TableParquetOptions::default(),
    ))
}

/// Create a [FileScanConfig] for scanning tuples of `schema` from `filename`.
pub fn local_file_config(schema: datatypes::SchemaRef, filename: &str) -> FileScanConfig {
    // TODO(George): think about parallel reads using partitioned file groups
    let pq_file = PartitionedFile::from_path(filename.to_string()).unwrap();

    FileScanConfig {
        object_store_url: object_store::ObjectStoreUrl::parse("file://").unwrap(),
        file_schema: Arc::clone(&schema),
        file_groups: vec![vec![pq_file]],
        statistics: Statistics::new_unknown(&schema),
        projection: None,
        limit: None,
        table_partition_cols: vec![],
        output_ordering: vec![],
    }
}

// think about parallel reads using partitioned file groups
pub fn local_filegroup_config(
    schema: datatypes::SchemaRef,
    filegroup: Vec<Vec<PartitionedFile>>,
) -> FileScanConfig {
    FileScanConfig {
        object_store_url: object_store::ObjectStoreUrl::parse("file://").unwrap(),
        file_schema: Arc::clone(&schema),
        file_groups: filegroup,
        statistics: Statistics::new_unknown(&schema),
        projection: None,
        limit: None,
        table_partition_cols: vec![],
        output_ordering: vec![],
    }
}

async fn process_sql_request(
    ctx: &SessionContext,
    item_id: u64,
) -> Result<(), Box<dyn error::Error>> {
    let sql = format!(
        "SELECT a.*, b.price, a.quantity * b.price as total
                        FROM orders a inner join prices b
                        ON a.item_id = b.item_id
                        and a.item_id = {}
                        ORDER by a.order_id",
        item_id
    );
    let logical_plan = ctx.state().create_logical_plan(sql.as_str()).await?;
    let physical_plan = ctx.state().create_physical_plan(&logical_plan).await?;

    SCHEDULER_INSTANCE
        .schedule_query(physical_plan, QueryInfo::default(), false)
        .await;
    Ok(())
}

async fn process_physical_fragment(
    fragment: QueryFragment,
    ctx: &SessionContext,
    abs_path_str: &str,
    id: u64,
    test: &str,
) {
    let query_id = fragment.query_id;
    let fragment_id = fragment.fragment_id;
    if fragment_id == 0 {
        assert!(fragment.parent_fragments.is_empty());
    }
    let process_plan = fragment.root.unwrap();
    let output_schema = process_plan.schema();
    let intermediate_output =
        format!("{abs_path_str}/src/example_data/test_{test}/query_{query_id}_fragment_{fragment_id}_pid_{id}.parquet");
    let context = ctx.state().task_ctx();
    let output_stream = physical_plan::execute_stream(process_plan, context).unwrap();

    spill_records_to_disk(
        &intermediate_output,
        output_stream,
        output_schema.clone(),
        10000,
        2,
        fragment.parent_fragments.is_empty(),
    )
    .await
    .unwrap();
    SCHEDULER_INSTANCE
        .finish_fragment(
            fragment_id,
            scheduler::QueryResult::ParquetExec(local_file_config(
                output_schema,
                intermediate_output.as_str(),
            )),
        )
        .await;
}

async fn spin_up(
    test: &str,
    id: u64,
    ctx: SessionContext,
    abs_path_str: String,
    live_for: std::time::Duration,
    pause_between_fragment: std::time::Duration,
) {
    let born = std::time::SystemTime::now();
    loop {
        std::thread::sleep(pause_between_fragment);
        if let Some(fragment) = SCHEDULER_INSTANCE.get_next_query_fragment().await {
            process_physical_fragment(fragment, &ctx, &abs_path_str, id, test).await;
        } else {
            if std::time::SystemTime::now().duration_since(born).unwrap() > live_for {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser;
    use crate::queue::{self, clear_queue};
    use crate::utils::*;
    use datafusion::arrow::array::RecordBatch;
    use datafusion::arrow::compute::kernels::concat;
    use datafusion::arrow::ipc;
    use datafusion::arrow::util::pretty;
    use datafusion::physical_plan;
    use datafusion::physical_plan::ExecutionPlanProperties;
    use serial_test::serial;
    use std::fs::File;

    #[tokio::test]
    #[serial]
    async fn spill_test() -> Result<(), Box<dyn error::Error>> {
        let abs_path = std::fs::canonicalize(".")?;
        let abs_path_str = abs_path.to_str().unwrap();

        // register the table
        let ctx = SessionContext::new();
        ctx.register_csv(
            "orders",
            "src/example_data/orders.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "prices",
            "src/example_data/prices.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "rand",
            "src/example_data/RandomData.csv",
            CsvReadOptions::new(),
        )
        .await?;

        // create a plan to run a SQL query
        let sql = "SELECT * from orders";
        let logical_plan = ctx.state().create_logical_plan(sql).await?;
        let pq = ctx.state().create_physical_plan(&logical_plan).await?;
        let ar = ctx.state().create_physical_plan(&logical_plan).await?;

        // get the recordbatch streams
        let test_schema = pq.schema();
        let context = ctx.state().task_ctx();
        let pq_stream = physical_plan::execute_stream(pq, context)?;

        // save the parquet to disk
        let pq_filename = format!("{}/src/example_data/spilltest/spill_test", abs_path_str);
        let fg = spill_records_to_disk(
            &pq_filename,
            pq_stream,
            test_schema.clone(),
            10000,
            1,
            false,
        )
        .await?;
        let pq_scan = scan_from_parquet(local_filegroup_config(test_schema.clone(), fg));
        // try executing the scan node and check that it works
        let context = ctx.state().task_ctx();
        let pq_scan_stream = physical_plan::execute_stream(pq_scan, context)?;
        let pq_read = pq_scan_stream.try_collect::<Vec<_>>().await?;
        pretty::print_batches(&pq_read)?;

        // save arrow to disk
        let context = ctx.state().task_ctx();
        let mut ar = physical_plan::execute_stream(ar, context)?;

        let arrow_file = format!(
            "{}/src/example_data/spilltest/spill_test.arrow",
            abs_path_str
        );
        let path = Path::new(&arrow_file);
        let file = File::create(path)?;
        let mut ar_writer = ipc::writer::StreamWriter::try_new(file, &test_schema)?;
        while let Some(rec_batch) = ar.try_next().await? {
            ar_writer.write(&rec_batch)?;
        }
        ar_writer.finish()?;

        // read in left side for comparisons
        let mut ar_read = Vec::<RecordBatch>::new();
        let left_file = File::open(path)?;
        let left_reader = ipc::reader::StreamReader::try_new(left_file, None)?;
        for rec_batch in left_reader {
            ar_read.push(rec_batch?);
        }

        // check tables are the same
        assert_eq!(
            concat::concat_batches(&test_schema, &ar_read)?,
            concat::concat_batches(&test_schema, &pq_read)?
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn spill_test_part() -> Result<(), Box<dyn error::Error>> {
        let abs_path = std::fs::canonicalize(".")?;
        let abs_path_str = abs_path.to_str().unwrap();

        // register the table
        let ctx = SessionContext::new();
        ctx.register_csv(
            "orders",
            "src/example_data/orders.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "prices",
            "src/example_data/prices.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "rand",
            "src/example_data/RandomData.csv",
            CsvReadOptions::new(),
        )
        .await?;

        // create a plan to run a SQL query
        let sql = "SELECT * from rand";
        let logical_plan = ctx.state().create_logical_plan(sql).await?;
        let pq = ctx.state().create_physical_plan(&logical_plan).await?;
        let ar = ctx.state().create_physical_plan(&logical_plan).await?;

        // get the recordbatch streams
        let test_schema = pq.schema();
        let context = ctx.state().task_ctx();
        let pq_stream = physical_plan::execute_stream(pq, context)?;

        // save the parquet to disk
        let pq_filename = format!(
            "{}/src/example_data/test_spill_test_part/spill_test",
            abs_path_str
        );
        let fg = spill_records_to_disk(
            &pq_filename,
            pq_stream,
            test_schema.clone(),
            250000,
            2,
            false,
        )
        .await?;
        let pq_scan = scan_from_parquet(local_filegroup_config(test_schema.clone(), fg.clone()));
        debug_assert!(pq_scan.output_partitioning().partition_count() == fg.len());

        // try executing the scan node and check that it works
        let context = ctx.state().task_ctx();
        let pq_scan_stream = physical_plan::execute_stream(pq_scan, context)?;
        let pq_read = pq_scan_stream.try_collect::<Vec<_>>().await?;

        // save arrow to disk
        let context = ctx.state().task_ctx();
        let mut ar = physical_plan::execute_stream(ar, context)?;

        let arrow_file = format!(
            "{}/src/example_data/test_spill_test_part/spill_test.arrow",
            abs_path_str
        );
        let path = Path::new(&arrow_file);
        let file = File::create(path)?;
        let mut ar_writer = ipc::writer::StreamWriter::try_new(file, &test_schema)?;
        while let Some(rec_batch) = ar.try_next().await? {
            ar_writer.write(&rec_batch)?;
        }
        ar_writer.finish()?;

        // // read in left side for comparisons
        let mut ar_read = Vec::<RecordBatch>::new();
        let left_file = File::open(path)?;
        let left_reader = ipc::reader::StreamReader::try_new(left_file, None)?;
        for rec_batch in left_reader {
            ar_read.push(rec_batch?);
        }

        // check tables are the same
        assert_eq!(
            concat::concat_batches(&test_schema, &ar_read)?.num_rows(),
            concat::concat_batches(&test_schema, &pq_read)?.num_rows()
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore] // ignore for now, hash probe and build does not give 2 root frags
    #[serial]
    async fn csv_query() -> Result<(), Box<dyn error::Error>> {
        clear_queue().await;
        let abs_path = std::fs::canonicalize(".")?;
        let abs_path_str = abs_path.to_str().unwrap();

        // register the table
        let ctx = SessionContext::new();
        ctx.register_csv(
            "orders",
            "src/example_data/orders.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "prices",
            "src/example_data/prices.csv",
            CsvReadOptions::new(),
        )
        .await?;

        // create a plan to run a SQL query
        let sql = "SELECT a.*, b.price, a.quantity * b.price as total
                         FROM orders a inner join prices b
                         ON a.item_id = b.item_id
                         and a.item_id = 6
                         ORDER by a.order_id";
        let logical_plan = ctx.state().create_logical_plan(sql).await?;
        let physical_plan = ctx.state().create_physical_plan(&logical_plan).await?;

        let mut fragments = parser::parse_into_fragments_wrapper(physical_plan, 0, 0, false).await;
        assert_eq!(fragments.len(), 3);

        // evaluate the two children first
        let context = ctx.state().task_ctx();
        let left_schema = fragments.get(&1).unwrap().root.clone().unwrap().schema();
        let left_path = fragments.get(&1).unwrap().parent_path_from_root.clone();
        let left =
            physical_plan::execute_stream(fragments.remove(&1).unwrap().root.unwrap(), context)?;
        let context = ctx.state().task_ctx();
        let right_schema = fragments.get(&2).unwrap().root.clone().unwrap().schema();
        let right_path = fragments.get(&2).unwrap().parent_path_from_root.clone();
        let right =
            physical_plan::execute_stream(fragments.remove(&2).unwrap().root.unwrap(), context)?;

        //spill to disk
        let left_pq_file = format!(
            "{}/src/example_data/test_csv_query/csv_query_left",
            abs_path_str
        );
        let right_pq_file = format!(
            "{}/src/example_data/test_csv_query/csv_query_right",
            abs_path_str
        );
        let left_fg =
            spill_records_to_disk(&left_pq_file, left, left_schema.clone(), 10000, 1, false)
                .await?;
        let right_fg =
            spill_records_to_disk(&right_pq_file, right, right_schema.clone(), 10000, 1, false)
                .await?;

        //read disk
        let root_fragment = fragments.remove(&0).unwrap();
        let root_exec = queue::update_plan_parent(
            root_fragment.root.unwrap(),
            &left_path[0],
            &scheduler::QueryResult::ParquetExec(local_filegroup_config(
                left_schema.clone(),
                left_fg,
            )),
        );
        let root_exec = queue::update_plan_parent(
            root_exec,
            &right_path[0],
            &scheduler::QueryResult::ParquetExec(local_filegroup_config(
                right_schema.clone(),
                right_fg,
            )),
        );

        let context = ctx.state().task_ctx();
        let csv_result = physical_plan::execute_stream(root_exec, context)?;
        pretty::print_batches(&csv_result.try_collect::<Vec<_>>().await?)?;

        Ok(())
    }

    #[tokio::test]
    async fn csv_query_queue_api_multi_async() -> Result<(), Box<dyn error::Error>> {
        clear_queue().await;
        let abs_path = std::fs::canonicalize(".")?;
        let abs_path_str = abs_path.to_str().unwrap();
        let abs_path_string = abs_path_str.to_string();

        // register the table
        let ctx = SessionContext::new();
        ctx.register_csv(
            "orders",
            "src/example_data/orders.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "prices",
            "src/example_data/prices.csv",
            CsvReadOptions::new(),
        )
        .await?;

        // create plans to run a SQL query
        for i in 1..11 {
            process_sql_request(&ctx, i).await?;
        }

        let mut handles = Vec::new();
        for i in 0..3 {
            let clone_path = abs_path_string.clone();
            let clone_ctx = ctx.clone();
            handles.push(tokio::spawn(async move {
                spin_up(
                    "csv_conc_async",
                    i,
                    clone_ctx,
                    clone_path,
                    std::time::Duration::from_secs(10),
                    std::time::Duration::from_millis(0),
                )
                .await;
            }));
        }

        for handle in handles {
            let _ = handle.await;
        }
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn csv_abort() -> Result<(), Box<dyn error::Error>> {
        clear_queue().await;
        let abs_path = std::fs::canonicalize(".")?;
        let abs_path_str = abs_path.to_str().unwrap();
        let abs_path_string = abs_path_str.to_string();

        // register the table
        let ctx = SessionContext::new();
        ctx.register_csv(
            "orders",
            "src/example_data/orders.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "prices",
            "src/example_data/prices.csv",
            CsvReadOptions::new(),
        )
        .await?;

        // create plans to run a SQL query
        for i in 1..11 {
            process_sql_request(&ctx, i).await?;
        }

        let mut handles = Vec::new();
        for i in 0..3 {
            let clone_path = abs_path_string.clone();
            let clone_ctx = ctx.clone();
            handles.push(tokio::spawn(async move {
                spin_up(
                    "csv_abort",
                    i,
                    clone_ctx,
                    clone_path,
                    std::time::Duration::from_secs(10),
                    std::time::Duration::from_millis(0),
                )
                .await;
            }));
        }

        SCHEDULER_INSTANCE.abort_query(1).await;
        SCHEDULER_INSTANCE.abort_query(3).await;
        SCHEDULER_INSTANCE.abort_query(5).await;
        SCHEDULER_INSTANCE.abort_query(7).await;
        SCHEDULER_INSTANCE.abort_query(9).await;
        SCHEDULER_INSTANCE.abort_query(12).await; // should not crash

        for handle in handles {
            let _ = handle.await;
        }
        Ok(())
    }
}
