use crate::generate_workflows::input_map::InputMap;
use std::collections::BTreeMap;

pub fn create_dependencies(
    input_map: &InputMap,
    workflow_name: String,
) -> BTreeMap<(String, String), Vec<(String, String)>> {
    input_map.get_service_dependencies(workflow_name.to_string())
}
