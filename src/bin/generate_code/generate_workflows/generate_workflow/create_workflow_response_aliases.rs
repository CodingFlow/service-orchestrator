use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_body_variables::BodyPropertyAlias;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::OperationSpec;
use crate::traversal::NestedNode;
use super::create_body_aliases::{create_body_aliases, AliasLocation};

pub fn create_workflow_response_aliases(
    iter: std::slice::Iter<'_, OperationSpec>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
) -> Vec<NestedNode<BodyPropertyAlias>> {
    iter.clone()
        .map(|operation_spec| {
            let workflow_name = operation_spec.operation_id.to_string();
            let namespace = (workflow_name, "response".to_string(), None, Location::Body);

            create_body_aliases(
                operation_spec.response_specs.first().unwrap().body.clone(),
                input_map,
                variable_aliases,
                namespace,
                AliasLocation::Destination,
            )
        })
        .collect()
}
