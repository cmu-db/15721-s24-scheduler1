use crate::{
    parser::{QueryFragment, QueryFragmentId},
    scheduler::{QueryResult, SCHEDULER_INSTANCE},
};
use datafusion::datasource::listing::PartitionedFile;
use datafusion::{config::TableParquetOptions, physical_plan::joins::HashJoinExec};
use datafusion::{
    datasource::physical_plan::{ArrowExec, FileScanConfig, ParquetExec},
    physical_plan::joins::HashBuildExec,
};

use super::debug_println;

use datafusion::physical_plan::ExecutionPlan;
use std::{collections::HashMap, sync::Arc, time::SystemTime};

/// Once a Execution plan has been parsed push all the fragments that can be scheduled onto the queue.
pub async fn add_fragments_to_scheduler(mut map: HashMap<QueryFragmentId, QueryFragment>) {
    let mut pending_fragments = SCHEDULER_INSTANCE.pending_fragments.write().await;
    let mut all_fragments = SCHEDULER_INSTANCE.all_fragments.write().await;
    for (&id, fragment) in map.iter_mut() {
        if fragment.child_fragments.is_empty() {
            debug_println!(
                "Pushing qid {} fid {} into pending_fragments",
                fragment.query_id,
                fragment.fragment_id
            );
            pending_fragments.push(id);
            fragment.enqueued_time = Some(SystemTime::now());
        }
    }
    debug_println!(
        "{} fragments available to be executed in pending queue",
        pending_fragments.len()
    );
    all_fragments.extend(map);
}

// Some assumptions have been made about priorites which may change
pub fn get_priority_from_fragment(fragment: &QueryFragment) -> i128 {
    let time = fragment
        .enqueued_time
        .unwrap()
        .elapsed()
        .expect("elapsed failed")
        .as_millis();

    // TODO Justify these magic constants and test them
    let priority = (fragment.query_priority as i128) * 100 + (time as i128) / 100;
    let cost_offset = match fragment.fragment_cost {
        Some(cost) => 30 - (cost as i128),
        None => 0,
    };
    priority + cost_offset * 20
}

/// Get the plan with the highest priorty from the queue.
pub async fn get_plan_from_queue() -> Option<QueryFragment> {
    let mut pending_fragments = SCHEDULER_INSTANCE.pending_fragments.write().await;
    let all_fragments = SCHEDULER_INSTANCE.all_fragments.read().await;

    let mut ref_id: Option<QueryFragmentId> = None;
    let mut ref_priority: i128 = 0;
    for fragment_id in pending_fragments.iter() {
        let frag = all_fragments.get(fragment_id).unwrap();
        let priority = get_priority_from_fragment(frag);
        if priority > ref_priority || ref_id.is_none() {
            ref_id = Some(*fragment_id);
            ref_priority = priority;
        }
    }
    if ref_id.is_none() {
        return None;
    } else {
        let _frag = all_fragments.get(&ref_id.unwrap()).unwrap();
        pending_fragments.retain(|x| *x != ref_id.unwrap());
    }

    let fragment = all_fragments.get(&ref_id.unwrap()).unwrap();

    debug_println!(
        "fetched 1 fragment, {} left available to be executed in pending queue",
        pending_fragments.len()
    );

    Some(fragment.clone())
}

pub fn update_plan_parent(
    root: Arc<dyn ExecutionPlan>,
    path: &[u32],
    query_result: &QueryResult,
) -> Arc<dyn ExecutionPlan> {
    if path.is_empty() {
        match query_result {
            QueryResult::ArrowExec(file_config) => return create_arrow_scan_node(file_config),
            QueryResult::ParquetExec(file_config) => return create_parquet_scan_node(file_config),
            QueryResult::HashBuildExec(result) => return HashBuildExec::get(result),
        }
    }

    let children: Vec<Arc<dyn ExecutionPlan>> = root.children();
    let mut new_children = Vec::<Arc<dyn ExecutionPlan>>::new();

    let mut i: u32 = 0;
    for child in children {
        if i != path[0] {
            new_children.push(child);
        } else {
            if path.len() == 2 {
                if let Some(node) = child.as_any().downcast_ref::<HashJoinExec>() {
                    let _probe_side = node.right().clone();
                    // return Arc::new(
                    //     HashProbeExec::try_new(
                    //         probe_side,
                    //         node.on,
                    //         node.filter,
                    //         &node.join_type,
                    //         node.projection,
                    //         node.mode,
                    //         node.null_equals_null,
                    //         //node.join_data,
                    //     )
                    //     .unwrap(),
                    // );
                }
            }
            new_children.push(update_plan_parent(child, &path[1..], query_result));
        }
        i += 1;
    }
    root.with_new_children(new_children).unwrap()
}

/// Marks the completion of the execution of the query fragment with
/// `fragment_id` with result `fragment_result`.
pub async fn finish_fragment(
    fragment_id: QueryFragmentId,
    fragment_result: QueryResult,
    intermediate_files: Vec<Vec<PartitionedFile>>,
) -> Vec<String> {
    let mut pending_fragments = SCHEDULER_INSTANCE.pending_fragments.write().await;
    let mut all_fragments = SCHEDULER_INSTANCE.all_fragments.write().await;
    let mut intermediate_file_pin = SCHEDULER_INSTANCE.intermediate_files.write().await;
    let parent_fragment_ids = all_fragments
        .get(&fragment_id)
        .unwrap()
        .parent_fragments
        .clone();

    let parent_fragment_paths = all_fragments
        .get(&fragment_id)
        .unwrap()
        .parent_path_from_root
        .clone();

    // these intermediate files belongs to the child_fragment which has been processed and should be deleted
    let child_fragment_intermediate_files = all_fragments
        .get(&fragment_id)
        .unwrap()
        .intermediate_files
        .clone()
        .into_iter();

    let mut to_delete: Vec<String> = vec![];

    for file in child_fragment_intermediate_files {
        match intermediate_file_pin.get_mut(&file) {
            None => {
                debug_println!("This is a intermediate file that wasn't recorded or undercounted")
            }
            Some(1) => {
                intermediate_file_pin.remove(&file);
                to_delete.push(file);
            }
            Some(pin_count) => {
                debug_assert!(*pin_count > 1);
                *pin_count -= 1;
            }
        }
    }

    let mut new_ids_to_push = vec![];
    for (i, id) in parent_fragment_ids.iter().enumerate() {
        let parent_fragment = all_fragments.get_mut(id).unwrap();

        let path = &parent_fragment_paths[i];
        let new_root = update_plan_parent(
            parent_fragment.root.clone().unwrap(),
            path,
            &fragment_result,
        );

        parent_fragment.root = Some(new_root);

        for partition in intermediate_files.clone().into_iter() {
            for file in partition.clone().into_iter() {
                *intermediate_file_pin
                    .entry(file.path().to_string())
                    .or_insert(0) += 1;
                parent_fragment
                    .intermediate_files
                    .insert(file.path().to_string());
            }
        }

        parent_fragment
            .child_fragments
            .retain(|x| *x != fragment_id);
        if parent_fragment.child_fragments.is_empty() {
            parent_fragment.enqueued_time = Some(SystemTime::now());
            new_ids_to_push.push(id);
        }
    }
    all_fragments.remove(&fragment_id);
    pending_fragments.extend(new_ids_to_push);
    debug_println!(
        "Updated finished fragments, {} left available to be executed in pending queue",
        pending_fragments.len()
    );

    to_delete
}

fn create_arrow_scan_node(file_config: &FileScanConfig) -> Arc<dyn ExecutionPlan> {
    Arc::new(ArrowExec::new(file_config.clone()))
}

fn create_parquet_scan_node(file_config: &FileScanConfig) -> Arc<dyn ExecutionPlan> {
    Arc::new(ParquetExec::new(
        file_config.clone(),
        None,
        None,
        TableParquetOptions::default(),
    ))
}

pub async fn abort_query(query_id: u64) {
    let mut all_fragments = SCHEDULER_INSTANCE.all_fragments.write().await;

    all_fragments.iter_mut().for_each(|x| {
        if x.1.query_id == query_id {
            x.1.aborted = true;
        }
    });
}

pub async fn clear_queue() {
    let mut all_fragments = SCHEDULER_INSTANCE.all_fragments.write().await;
    let mut pending_fragments = SCHEDULER_INSTANCE.pending_fragments.write().await;
    let mut job_status = SCHEDULER_INSTANCE.job_status.write().await;
    let mut intermediate_files = SCHEDULER_INSTANCE.intermediate_files.write().await;

    all_fragments.clear();
    pending_fragments.clear();
    job_status.clear();
    intermediate_files.clear();
}
#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::queue::*;
    use datafusion::arrow::datatypes::{DataType, Field, Schema, SchemaRef};
    use datafusion::datasource::{empty::EmptyTable, DefaultTableSource};
    use datafusion::execution::context::SessionState;

    use datafusion::execution::object_store::ObjectStoreUrl;
    use datafusion::logical_expr::LogicalPlanBuilder;
    use datafusion::physical_plan::joins::NestedLoopJoinExec;
    use datafusion::physical_plan::ExecutionPlan;
    use datafusion::physical_plan::{
        coalesce_batches::CoalesceBatchesExec, empty::EmptyExec, filter::FilterExec,
    };
    use datafusion::physical_planner::{DefaultPhysicalPlanner, PhysicalPlanner};
    use datafusion_common::Result;
    use datafusion_common::Statistics;
    use datafusion_expr::JoinType;
    use datafusion_expr::{col, lit, LogicalPlan};

    use more_asserts as ma;
    use serial_test::serial;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::sync::Arc;

    async fn create_physical_plan(logical_plan: LogicalPlan) -> Result<Arc<dyn ExecutionPlan>> {
        // Set default for all context
        let physical_planner = DefaultPhysicalPlanner::default();
        let session_state =
            SessionState::new_with_config_rt(Default::default(), Default::default());

        // Create the physical plan from the logical plan
        let execution_plan = physical_planner
            .create_physical_plan(&logical_plan, &session_state)
            .await?;

        Ok(execution_plan)
    }

    async fn build_toy_physical_plan() -> Result<Arc<dyn ExecutionPlan>> {
        // create a logical table source
        let schema = Schema::new(vec![
            Field::new("id", DataType::Int32, true),
            Field::new("name", DataType::Utf8, true),
        ]);
        let empty_table = Arc::new(EmptyTable::new(SchemaRef::new(schema)));
        let table_source = Arc::new(DefaultTableSource::new(empty_table));

        // optional projection
        let projection = None;

        // create a LogicalPlanBuilder for a table scan
        let builder = LogicalPlanBuilder::scan("person", table_source, projection)?;

        // perform a filter operation and build the plan
        let logical_plan = builder
            .filter(col("id").gt(lit(500)))? // WHERE id > 500
            .build()?;

        create_physical_plan(logical_plan).await
    }

    fn validate_toy_physical_plan_structure(root_node: &Arc<dyn ExecutionPlan>) {
        root_node
            .as_any()
            .downcast_ref::<CoalesceBatchesExec>()
            .unwrap();

        let node0_children = root_node.children();
        assert_eq!(node0_children.len(), 1);

        let node1 = node0_children.first().unwrap();
        node1.as_any().downcast_ref::<FilterExec>().unwrap();

        let node1_children = node1.children();
        assert_eq!(node1_children.len(), 1);

        let node2 = node1_children.first().unwrap();
        node2.as_any().downcast_ref::<EmptyExec>().unwrap();

        let node2_children = node2.children();
        assert!(node2_children.is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn sanity_check() {
        clear_queue().await;
        let physical_plan = build_toy_physical_plan().await.unwrap();
        println!("Physical Plan: {:#?}", physical_plan);
        assert_eq!(physical_plan.children().len(), 1);
        validate_toy_physical_plan_structure(&physical_plan);

        // Returns a hash map from query fragment ID to physical plan fragment structs
        let fragment = QueryFragment {
            fragment_id: 0,
            query_id: 0,
            root: Some(physical_plan),
            parent_path_from_root: vec![],
            child_fragments: vec![],
            parent_fragments: vec![],
            query_priority: 0,
            enqueued_time: None,
            fragment_cost: None,
            intermediate_files: HashSet::<String>::new(),
            aborted: false,
        };
        let mut map: HashMap<QueryFragmentId, QueryFragment> = HashMap::new();
        map.insert(0, fragment);
        add_fragments_to_scheduler(map).await;
        let queued_fragment = get_plan_from_queue().await.unwrap();
        assert!(queued_fragment.root.is_some());
        ma::assert_ge!(queued_fragment.query_id, 0);
        ma::assert_ge!(queued_fragment.fragment_id, 0);

        let frag_node0 = queued_fragment.root.clone().unwrap();
        validate_toy_physical_plan_structure(&frag_node0);

        assert!(queued_fragment.child_fragments.is_empty());
        assert!(queued_fragment.parent_fragments.is_empty());
        assert!(queued_fragment.parent_path_from_root.is_empty());
        assert!(queued_fragment.enqueued_time.is_some());

        assert_eq!(SCHEDULER_INSTANCE.pending_fragments.read().await.len(), 0);
    }

    async fn build_basic_physical_plan() -> Result<Arc<dyn ExecutionPlan>> {
        // create a logical table source
        let price_schema = Schema::new(vec![
            Field::new("item_id", DataType::Int32, true),
            Field::new("price", DataType::Utf8, true),
        ]);
        let order_schema = Schema::new(vec![
            Field::new("order_id", DataType::Int32, true),
            Field::new("item_id", DataType::Int32, true),
            Field::new("quantity", DataType::Int32, true),
        ]);
        let price_table = Arc::new(EmptyTable::new(SchemaRef::new(price_schema)));
        let price_table_source = Arc::new(DefaultTableSource::new(price_table));

        let order_table = Arc::new(EmptyTable::new(SchemaRef::new(order_schema)));
        let order_table_source = Arc::new(DefaultTableSource::new(order_table));

        let exprs = vec![col("price.item_id").eq(col("order.item_id"))];

        // create a LogicalPlanBuilder for a table scan
        let right_plan = LogicalPlanBuilder::scan("price", price_table_source, None)?.build()?;

        let plan = LogicalPlanBuilder::scan("order", order_table_source, None)?
            .join_on(right_plan, JoinType::Inner, exprs)?
            .build()?;

        create_physical_plan(plan).await
    }

    fn validate_basic_physical_plan_structure(root_node: &Arc<dyn ExecutionPlan>) {
        root_node
            .as_any()
            .downcast_ref::<NestedLoopJoinExec>()
            .unwrap();

        let node0_children = root_node.children();
        assert_eq!(node0_children.len(), 2);

        let node1 = node0_children.first().unwrap();
        node1.as_any().downcast_ref::<EmptyExec>().unwrap();

        let node2 = node0_children.get(1).unwrap();
        node2.as_any().downcast_ref::<EmptyExec>().unwrap();

        assert_eq!(node1.children().len(), 0);
        assert_eq!(node2.children().len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn basic_test() {
        clear_queue().await;
        let physical_plan = build_basic_physical_plan().await.unwrap();
        println!("Physical Plan: {:#?}", physical_plan);
        validate_basic_physical_plan_structure(&physical_plan);

        // Returns a hash map from query fragment ID to physical plan fragment structs
        let fragment_map = parse_into_fragments_wrapper(physical_plan, 0, 0, true).await;

        add_fragments_to_scheduler(fragment_map).await;
        assert_eq!(SCHEDULER_INSTANCE.pending_fragments.read().await.len(), 2);

        let mut child_fragment_vec = Vec::<QueryFragment>::new();

        let mut queued_fragment = get_plan_from_queue().await.unwrap();
        assert!(queued_fragment.root.is_some());
        child_fragment_vec.push(queued_fragment);
        queued_fragment = get_plan_from_queue().await.unwrap();
        assert!(queued_fragment.root.is_some());
        child_fragment_vec.push(queued_fragment);
        assert!(get_plan_from_queue().await.is_none());

        assert_eq!(SCHEDULER_INSTANCE.pending_fragments.read().await.len(), 0);

        finish_fragment(
            child_fragment_vec[0].fragment_id,
            QueryResult::ArrowExec(FileScanConfig {
                object_store_url: ObjectStoreUrl::parse("https://example.net").unwrap(),
                file_schema: SchemaRef::new(Schema::empty()),
                file_groups: vec![],
                statistics: Statistics::new_unknown(&Schema::empty()),
                projection: None,
                limit: None,
                table_partition_cols: vec![],
                output_ordering: vec![],
            }),
            vec![vec![]],
        )
        .await;

        assert!(get_plan_from_queue().await.is_none());
        assert_eq!(SCHEDULER_INSTANCE.pending_fragments.read().await.len(), 0);

        finish_fragment(
            child_fragment_vec[1].fragment_id,
            QueryResult::ArrowExec(FileScanConfig {
                object_store_url: ObjectStoreUrl::parse("https://example.net").unwrap(),
                file_schema: SchemaRef::new(Schema::empty()),
                file_groups: vec![],
                statistics: Statistics::new_unknown(&Schema::empty()),
                projection: None,
                limit: None,
                table_partition_cols: vec![],
                output_ordering: vec![],
            }),
            vec![vec![]],
        )
        .await;

        let root_fragments = get_plan_from_queue().await.unwrap();

        assert!(get_plan_from_queue().await.is_none());
        assert_eq!(SCHEDULER_INSTANCE.pending_fragments.read().await.len(), 0);
        let mut num_child = 0;
        for child in root_fragments.root.unwrap().children() {
            child.as_any().downcast_ref::<ArrowExec>().unwrap();
            num_child += 1;
        }
        assert_eq!(num_child, 2);
    }

    #[tokio::test]
    #[serial]
    async fn abort_test() {
        clear_queue().await;
        let physical_plan = build_basic_physical_plan().await.unwrap();
        validate_basic_physical_plan_structure(&physical_plan);

        // Returns a hash map from query fragment ID to physical plan fragment structs
        let fragment_map = parse_into_fragments_wrapper(physical_plan, 0, 0, true).await;

        add_fragments_to_scheduler(fragment_map).await;
        assert_eq!(SCHEDULER_INSTANCE.pending_fragments.read().await.len(), 2);

        let mut child_fragment_vec = Vec::<QueryFragment>::new();

        let queued_fragment = get_plan_from_queue().await.unwrap();
        assert!(queued_fragment.root.is_some());
        let q_id = queued_fragment.query_id;
        child_fragment_vec.push(queued_fragment);
        abort_query(q_id).await;

        let mut expected_abort = 0;
        for (_, value) in SCHEDULER_INSTANCE.all_fragments.read().await.iter() {
            assert!(value.query_id != 0 || value.query_id == 0 && value.aborted);
            if value.query_id == 0 && value.aborted {
                expected_abort += 1;
            }
        }

        debug_println!("{}", expected_abort);
        assert!(expected_abort == 3);
    }
}
