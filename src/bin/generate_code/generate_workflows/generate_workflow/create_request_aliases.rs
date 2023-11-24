use super::build_service_call_view_data::generate_response_variables::{AliasType, ResponseAlias};
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::parse_schema::ParsedSchema;
use crate::traversal::{map_nested_node, NestedNode};

pub fn create_request_aliases(
    request_body: NestedNode<ParsedSchema>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    namespace: (String, String, Option<String>, Location),
) -> NestedNode<ResponseAlias> {
    map_nested_node(
        request_body,
        |parent_schema_node, (input_map, variable_aliases, alias_accumulator, namespace)| {
            if let None = parent_schema_node.children {
                let mut map_to_key = alias_accumulator.to_vec();

                map_to_key.push(parent_schema_node.current.name.unwrap());

                let alias = input_map.create_variable_alias(namespace.clone(), map_to_key);

                ResponseAlias {
                    name: Some(alias.original_name),
                    variable_alias: alias.alias,
                    schema_type: parent_schema_node.current.schema_type,
                    alias_type: AliasType::Field,
                }
            } else {
                if let Some(name) = parent_schema_node.current.name.clone() {
                    alias_accumulator.push(name);
                }

                let alias = variable_aliases.create_alias();

                ResponseAlias {
                    name: parent_schema_node.current.name,
                    variable_alias: alias,
                    schema_type: parent_schema_node.current.schema_type,
                    alias_type: AliasType::Struct,
                }
            }
        },
        |_, (_, _, alias_accumulator, _)| {
            alias_accumulator.pop();
        },
        &mut (input_map, variable_aliases, vec![], namespace),
    )
}
