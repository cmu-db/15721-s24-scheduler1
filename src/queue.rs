use crate::{
    parser::{PhysicalPlanFragment, QueryFragmentId},
    scheduler::SCHEDULER_INSTANCE,
};
use datafusion::datasource::physical_plan::{ArrowExec, FileScanConfig};
use datafusion::physical_plan::ExecutionPlan;
use std::{collections::HashMap, sync::Arc};

pub fn add_fragments_to_scheduler(map: HashMap<QueryFragmentId, PhysicalPlanFragment>) {
    let mut scheduler_instance = SCHEDULER_INSTANCE.lock().unwrap();
    for (&id, fragment) in map.iter() {
        if fragment.child_fragments.is_empty() {
            scheduler_instance.pending_fragments.push(id);
        }
    }
    scheduler_instance.all_fragments.extend(map);
}

pub fn get_plan_from_queue() -> Option<Arc<dyn ExecutionPlan>> {
    let mut scheduler_instance = SCHEDULER_INSTANCE.lock().unwrap();
    let fragment_id = scheduler_instance.pending_fragments.pop()?;

    let new_plan = &scheduler_instance
        .all_fragments
        .get(&fragment_id)
        .unwrap()
        .root;
    new_plan.clone()
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
