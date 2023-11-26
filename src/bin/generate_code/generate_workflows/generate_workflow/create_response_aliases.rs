use super::build_service_call_view_data::generate_body_variables::BodyPropertyAlias;
use super::create_body_aliases::create_body_aliases;
use super::create_body_aliases::AliasLocation;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::OperationSpec;
use crate::traversal::NestedNode;

pub fn create_response_aliases(
    iter: std::slice::Iter<'_, OperationSpec>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
) -> Vec<NestedNode<BodyPropertyAlias>> {
    iter.clone()
        .map(|operation_spec| {
            let namespace = (
                workflow_name.to_string(),
                operation_spec.spec_name.to_string(),
                Some(operation_spec.operation_id.to_string()),
                Location::Body,
            );

            create_body_aliases(
                operation_spec.response_specs.first().unwrap().body.clone(),
                input_map,
                variable_aliases,
                namespace,
                AliasLocation::Source,
            )
        })
        .collect()
}
