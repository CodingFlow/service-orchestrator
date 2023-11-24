use super::build_service_call_view_data::generate_response_variables::ResponseAlias;
use super::create_body_aliases::create_body_aliases;
use super::create_body_aliases::AliasLocation;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::parse_schema::ParsedSchema;
use crate::traversal::NestedNode;

pub fn create_workflow_request_aliases(
    request_body: NestedNode<ParsedSchema>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    operation_id: String,
) -> NestedNode<ResponseAlias> {
    let namespace = (operation_id, "response".to_string(), None, Location::Body);

    create_body_aliases(
        request_body,
        input_map,
        variable_aliases,
        namespace,
        AliasLocation::Source,
    )
}
