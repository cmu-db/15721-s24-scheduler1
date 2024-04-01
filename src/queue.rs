use crate::{
    parser::{PhysicalPlanFragment, QueryFragmentId},
    scheduler::SCHEDULER_INSTANCE,
};
use datafusion::datasource::physical_plan::{ArrowExec, FileScanConfig};

use datafusion::physical_plan::ExecutionPlan;
use std::{collections::HashMap, sync::Arc, time::SystemTime};

/// Once a Execution plan has been parsed push all the fragments that can be scheduled onto the queue.
pub fn add_fragments_to_scheduler(mut map: HashMap<QueryFragmentId, PhysicalPlanFragment>) {
    let mut scheduler_instance = SCHEDULER_INSTANCE.lock().unwrap();
    for (&id, fragment) in map.iter_mut() {
        if fragment.child_fragments.is_empty() {
            scheduler_instance.pending_fragments.push(id);
            fragment.enqueued_time = Some(SystemTime::now());
        }
    }
    scheduler_instance.all_fragments.extend(map);
}

// Some assumptions have been made about priorites which may change
pub fn get_priority_from_fragment(fragment: &PhysicalPlanFragment) -> i128 {
    let time = fragment
        .enqueued_time
        .unwrap()
        .elapsed()
        .expect("elapsed failed")
        .as_millis();

    // TODO Justify these magic constants and test them
    let priority = (fragment.query_priority as i128) * 100 + (time as i128) / 100;
    let cost_offset = match fragment.fragment_cost {
        Some(cost) => 100 - (cost as i128) / 1000,
        None => 0,
    };
    priority + cost_offset
}

pub fn get_plan_from_queue() -> Option<PhysicalPlanFragment> {
    let mut scheduler_instance = SCHEDULER_INSTANCE.lock().unwrap();

    let mut ref_id: Option<QueryFragmentId> = None;
    let mut ref_priority: i128 = 0;
    for fragment_id in &scheduler_instance.pending_fragments {
        let frag = scheduler_instance.all_fragments.get(&fragment_id).unwrap();
        let priority = get_priority_from_fragment(frag);
        if priority > ref_priority || ref_id.is_none() {
            ref_id = Some(*fragment_id);
            ref_priority = priority;
        }
    }
    if ref_id.is_none() {
        return None;
    } else {
        scheduler_instance
            .pending_fragments
            .retain(|x| *x != ref_id.unwrap());
    }

    let fragment = scheduler_instance
        .all_fragments
        .get(&ref_id.unwrap())
        .unwrap();
    Some(fragment.clone())
}

pub fn update_plan_parent(
    root: Arc<dyn ExecutionPlan>,
    path: &[u32],
    file_config: &FileScanConfig,
) -> Arc<dyn ExecutionPlan> {
    if path.is_empty() {
        return create_arrow_scan_node(&root, file_config);
    }

    let children: Vec<Arc<dyn ExecutionPlan>> = root.children();
    let mut new_children = Vec::<Arc<dyn ExecutionPlan>>::new();

    let mut i: u32 = 0;
    for child in children {
        if i != path[0] {
            new_children.push(child);
        } else {
            new_children.push(update_plan_parent(child, &path[1..], file_config));
        }
        i += 1;
    }
    root.with_new_children(new_children).unwrap()
}

pub fn finish_fragment(child_fragment_id: QueryFragmentId, file_config: FileScanConfig) {
    let mut scheduler_instance = SCHEDULER_INSTANCE.lock().unwrap();
    let parent_fragment_ids = scheduler_instance
        .all_fragments
        .get(&child_fragment_id)
        .unwrap()
        .parent_fragments
        .clone();

    let parent_fragment_paths = scheduler_instance
        .all_fragments
        .get(&child_fragment_id)
        .unwrap()
        .parent_path_from_root
        .clone();

    let mut new_ids_to_push = vec![];
    for (i, id) in parent_fragment_ids.iter().enumerate() {
        let parent_fragment = scheduler_instance.all_fragments.get_mut(id).unwrap();

        let path = &parent_fragment_paths[i];
        let new_root =
            update_plan_parent(parent_fragment.root.clone().unwrap(), path, &file_config);

        parent_fragment.root = Some(new_root);

        parent_fragment
            .child_fragments
            .retain(|x| *x != child_fragment_id);
        if parent_fragment.child_fragments.is_empty() {
            parent_fragment.enqueued_time = Some(SystemTime::now());
            new_ids_to_push.push(id);
        }
    }
    scheduler_instance.all_fragments.remove(&child_fragment_id);
    scheduler_instance.pending_fragments.extend(new_ids_to_push);
}

fn create_arrow_scan_node(
    _plan: &Arc<dyn ExecutionPlan>,
    file_config: &FileScanConfig,
) -> Arc<dyn ExecutionPlan> {
    Arc::new(ArrowExec::new(file_config.clone()))
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::queue::*;
    use datafusion::arrow::datatypes::{DataType, Field, Schema, SchemaRef};
    use datafusion::datasource::{empty::EmptyTable, DefaultTableSource};
    use datafusion::execution::context::SessionState;
    use datafusion::logical_expr::JoinType;
    use datafusion::logical_expr::LogicalPlanBuilder;
    use datafusion::physical_plan::ExecutionPlan;
    use datafusion::physical_plan::{
        coalesce_batches::CoalesceBatchesExec, empty::EmptyExec, filter::FilterExec,
        joins::NestedLoopJoinExec,
    };
    use datafusion::physical_planner::{DefaultPhysicalPlanner, PhysicalPlanner};
    use datafusion_common::Result;
    use datafusion_expr::{col, lit, LogicalPlan};
    use datafusion_proto::protobuf::PhysicalScalarUdfNode;
    use more_asserts as ma;
    use std::collections::HashMap;
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
    async fn sanity_check() {
        let physical_plan = build_toy_physical_plan().await.unwrap();
        println!("Physical Plan: {:#?}", physical_plan);
        assert_eq!(physical_plan.children().len(), 1);
        validate_toy_physical_plan_structure(&physical_plan);

        // Returns a hash map from query fragment ID to physical plan fragment structs
        let fragment = PhysicalPlanFragment {
            fragment_id: 0,
            query_id: 0,
            root: Some(physical_plan),
            parent_path_from_root: vec![],
            child_fragments: vec![],
            parent_fragments: vec![],
            query_priority: 0,
            enqueued_time: None,
            fragment_cost: None,
        };
        let mut map: HashMap<QueryFragmentId, PhysicalPlanFragment> = HashMap::new();
        map.insert(0, fragment);
        add_fragments_to_scheduler(map);
        let queued_fragment = get_plan_from_queue().unwrap();
        assert!(!queued_fragment.root.is_none());
        ma::assert_ge!(queued_fragment.query_id, 0);
        ma::assert_ge!(queued_fragment.fragment_id, 0);

        let frag_node0 = queued_fragment.root.clone().unwrap();
        validate_toy_physical_plan_structure(&frag_node0);

        assert!(queued_fragment.child_fragments.is_empty());
        assert!(queued_fragment.parent_fragments.is_empty());
        assert!(queued_fragment.parent_path_from_root.is_empty());
        assert!(queued_fragment.enqueued_time.is_some());
    }
}
