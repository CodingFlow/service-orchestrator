use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_response_variables::{ResponseAlias, AliasType};
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::OperationSpec;
use crate::traversal::map_nested_node;
use crate::traversal::NestedNode;

pub fn create_workflow_response_aliases(
    iter: std::slice::Iter<'_, OperationSpec>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
) -> Vec<NestedNode<ResponseAlias>> {
    iter.clone()
        .map(|operation_spec| {
            add_nested_response_aliases(
                operation_spec,
                input_map,
                variable_aliases,
                workflow_name.to_string(),
            )
        })
        .collect()
}

fn add_nested_response_aliases(
    operation_spec: &OperationSpec,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
) -> NestedNode<ResponseAlias> {
    // TODO: handle more than one status code

    map_nested_node(
        operation_spec.response_specs.first().unwrap().body.clone(),
        |parent_schema_node, (input_map, variable_aliases, alias_accumulator, namespace)| {
            if let None = parent_schema_node.children {
                let mut map_to_key = alias_accumulator.to_vec();

                map_to_key.push(parent_schema_node.current.name.clone().unwrap());

                let alias = input_map.get_variable_alias(namespace.clone(), map_to_key);

                ResponseAlias {
                    name: parent_schema_node.current.name,
                    variable_alias: alias,
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
        &mut (
            input_map,
            variable_aliases,
            vec![],
            (workflow_name, "response".to_string(), None, Location::Body),
        ),
    )
}
