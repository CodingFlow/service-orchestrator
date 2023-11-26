use super::build_service_call_view_data::generate_body_variables::AliasType;
use super::build_service_call_view_data::generate_body_variables::BodyPropertyAlias;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::parse_schema::ParsedSchema;
use crate::traversal::map_nested_node;
use crate::traversal::NestedNode;

pub enum AliasLocation {
    Source,
    Destination,
}

pub fn create_body_aliases(
    body: NestedNode<ParsedSchema>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    namespace: (String, String, Option<String>, Location),
    alias_location: AliasLocation,
) -> NestedNode<BodyPropertyAlias> {
    // TODO: handle more than one status code

    map_nested_node(
        body,
        |parent_schema_node,
         (input_map, variable_aliases, alias_accumulator, namespace, alias_location)| {
            let ParsedSchema { name, schema_type } = parent_schema_node.current;

            if let None = parent_schema_node.children {
                let mut map_to_key = alias_accumulator.to_vec();

                map_to_key.push(name.clone().unwrap());

                let variable_alias = match alias_location {
                    AliasLocation::Source => {
                        input_map
                            .create_variable_alias(namespace.clone(), map_to_key)
                            .alias
                    }
                    AliasLocation::Destination => {
                        input_map.get_variable_alias(namespace.clone(), map_to_key)
                    }
                };

                BodyPropertyAlias {
                    name,
                    variable_alias,
                    schema_type,
                    alias_type: AliasType::Field,
                }
            } else {
                if let Some(name) = name.clone() {
                    alias_accumulator.push(name);
                }

                let variable_alias = variable_aliases.create_alias();

                BodyPropertyAlias {
                    name,
                    variable_alias,
                    schema_type,
                    alias_type: AliasType::Struct,
                }
            }
        },
        |_, (_, _, alias_accumulator, _, _)| {
            alias_accumulator.pop();
        },
        &mut (
            input_map,
            variable_aliases,
            vec![],
            namespace,
            alias_location,
        ),
    )
}
