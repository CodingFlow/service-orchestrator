use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_response_variables::{ServiceResponseAlias, AliasType};
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::{InputMap, InputMapBehavior};
use crate::parse_specs::OperationSpec;
use crate::traversal::traverse_nested_type;
use crate::traversal::NestedNode;

pub fn create_workflow_response_aliases(
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

    traverse_nested_type(
        operation_spec.response_specs.first().unwrap().body.clone(),
        |response_schema, (input_map, variable_aliases, alias_accumulator, namespace)| {
            if let None = response_schema.properties {
                let mut map_to_key = alias_accumulator.to_vec();

                map_to_key.push(response_schema.name.clone().unwrap());

                let alias = input_map.get_variable_alias(namespace.clone(), map_to_key);

                ServiceResponseAlias {
                    name: response_schema.name,
                    variable_alias: alias,
                    schema_type: response_schema.schema_type,
                    alias_type: AliasType::Field,
                }
            } else {
                if let Some(name) = response_schema.name.clone() {
                    alias_accumulator.push(name);
                }

                let alias = variable_aliases.create_alias();

                ServiceResponseAlias {
                    name: response_schema.name,
                    variable_alias: alias,
                    schema_type: response_schema.schema_type,
                    alias_type: AliasType::Struct,
                }
            }
        },
        |_, _, _| {},
        |schema| schema.properties,
        |_, (_, _, alias_accumulator, _)| {
            alias_accumulator.pop();
        },
        &mut (
            input_map,
            variable_aliases,
            vec![],
            (workflow_name, "response".to_string(), None),
        ),
    )
}
