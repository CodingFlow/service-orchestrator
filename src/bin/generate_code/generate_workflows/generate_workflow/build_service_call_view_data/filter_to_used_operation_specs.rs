use crate::generate_workflows::input_map::InputMap;
use crate::parse_specs::OperationSpec;

pub fn filter_to_used_operation_specs(
    workflow_name: String,
    operation_specs: Vec<OperationSpec>,
    input_map: &InputMap,
) -> Vec<OperationSpec> {
    let service_operation_names = input_map.get_workflow_services_operations_ids(workflow_name);

    operation_specs
        .into_iter()
        .filter(|spec| {
            (&service_operation_names)
                .into_iter()
                .any(|service_operation_name| {
                    *service_operation_name
                        == (spec.spec_name.to_string(), spec.operation_id.to_string())
                })
        })
        .collect()
}
