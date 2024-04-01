#[cfg(test)]
mod tests {
    use crate::parser;
    use datafusion::arrow::array::RecordBatch;
    use datafusion::arrow::ipc;
    use datafusion::arrow::util::pretty;
    use datafusion::physical_plan;
    use datafusion::prelude::*;
    use futures::stream::TryStreamExt;
    use std::fs::File;
    use std::path::Path;

    #[tokio::test]
    async fn csv_query() -> datafusion::error::Result<()> {
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
        let sql = "SELECT a.*, b.price from orders a inner join prices b on a.item_id = b.item_id order by a.order_id";
        let logical_plan = ctx.state().create_logical_plan(sql).await?;
        let physical_plan = ctx.state().create_physical_plan(&logical_plan).await?;

        let mut fragments = parser::parse_into_fragments_wrapper(physical_plan, 0).await;
        assert_eq!(fragments.len(), 3);
        let mut keys: Vec<u64> = vec![];
        for key in fragments.keys() {
            keys.push(*key);
        }

        // evaluate the two children first
        let context = ctx.state().task_ctx();
        let left_schema = fragments
            .get(&keys[1])
            .unwrap()
            .root
            .clone()
            .unwrap()
            .schema();
        let left = physical_plan::execute_stream(
            fragments.remove(&keys[1]).unwrap().root.unwrap(),
            context,
        )?;
        let context = ctx.state().task_ctx();
        let right = physical_plan::execute_stream(
            fragments.remove(&keys[2]).unwrap().root.unwrap(),
            context,
        )?;

        // TODO: George needs to stream record batch to disk without collecting
        let left_result = left.try_collect::<Vec<_>>().await.unwrap();
        let right_result = right.try_collect::<Vec<_>>().await.unwrap();

        // save the lhs for now,
        let mut left_read = Vec::<RecordBatch>::new();
        {
            let path = Path::new("src/example_data/left.ipc");
            let display = path.display();

            let file = match File::create(path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };

            let mut left_writer = ipc::writer::StreamWriter::try_new(file, &left_schema)?;
            for rec_batch in left_result.iter() {
                left_writer.write(rec_batch)?;
            }

            let left_file = match File::open(path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };

            let left_reader = ipc::reader::StreamReader::try_new(left_file, None)?;
            for rec_batch in left_reader {
                left_read.push(rec_batch.unwrap());
            }
        }

        assert!(left_result == left_read);

        pretty::print_batches(&left_result)?;
        pretty::print_batches(&right_result)?;

        Ok(())
    }
}
