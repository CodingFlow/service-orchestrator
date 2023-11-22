use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::variable_aliases::Location;
use crate::generate_workflows::input_map::InputMap;
use crate::parse_specs::OperationSpec;
use crate::traversal::{map_nested_node, NestedNode};

use super::generate_response_variables::{AliasType, ServiceResponseAlias};

pub fn create_response_aliases(
    iter: std::slice::Iter<'_, OperationSpec>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
) -> Vec<NestedNode<ServiceResponseAlias>> {
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
) -> NestedNode<ServiceResponseAlias> {
    // TODO: handle more than one status code

    map_nested_node(
        operation_spec.response_specs.first().unwrap().body.clone(),
        |parent_schema_node, (input_map, variable_aliases, alias_accumulator, namespace)| {
            if let None = parent_schema_node.children {
                let mut map_to_key = alias_accumulator.to_vec();

                map_to_key.push(parent_schema_node.current.name.unwrap());

                let alias = input_map.create_variable_alias(namespace.clone(), map_to_key);

                ServiceResponseAlias {
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

                ServiceResponseAlias {
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
            (
                workflow_name,
                operation_spec.spec_name.to_string(),
                Some(operation_spec.operation_id.to_string()),
                Location::Body,
            ),
        ),
    )
}
