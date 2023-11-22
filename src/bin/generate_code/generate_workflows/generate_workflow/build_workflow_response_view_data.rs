use super::super::input_map::InputMap;
use super::build_service_call_view_data::generate_response_variables::ResponseAlias;
use super::create_workflow_response_aliases::create_workflow_response_aliases;
use super::variables::VariableAliases;
use crate::parse_specs::OperationSpec;
use crate::traversal::NestedNode;

pub fn build_workflow_response_view_data(
    operation_spec: &OperationSpec,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
) -> Vec<NestedNode<ResponseAlias>> {
    create_workflow_response_aliases(
        vec![operation_spec.clone()].iter(),
        input_map,
        variable_aliases,
        operation_spec.operation_id.to_string(),
    )
}
