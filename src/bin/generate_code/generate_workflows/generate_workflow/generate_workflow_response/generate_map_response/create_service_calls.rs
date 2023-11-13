mod variables;

use std::collections::BTreeMap;

use codegen::Function;

use crate::{
    generate_workflows::input_map::{InputMap, InputMapBehavior, Variable},
    parse_specs::{get_operation_specs, OperationSpec, SpecType},
    traversal::{traverse_nested_type, NestedNode},
};

use self::variables::VariableAliases;

struct ServiceCodeGenerationInfo {
    future_variable_name: String,
    response_struct_name: String,
    response_aliases: NestedNode<String>,
    dependent_service_names: Vec<String>,
}

pub fn create_service_calls(
    function: &mut Function,
    mut input_map: InputMap,
    workflow_name: String,
) {
    let operation_specs = get_operation_specs(SpecType::Service);

    // Need to create variable aliases for service responses
    // before referencing them in service requests

    let used_operation_specs =
        filter_to_used_operation_specs(workflow_name, operation_specs, input_map);

    let variable_aliases = VariableAliases::new();

    let service_operation_specs = build_lookup_map(used_operation_specs, variable_aliases);

    // determine_dependencies();

    // create_response_struct();

    // create_request_response_futures(); // depends on dependencies

    // // streams depends only on futures. Enums depend on response structs and dependencies.
    // // Create enums as part of output: per future, create stream & enum.depends on response structs and dependencies.
    // create_streams_and_enums();
    // create_service_results_for_workflow_response(); // depends on enums, response structs, dependencies

    // generate_all_services() // actually "print" as generated code: keep this dumb.
}

fn filter_to_used_operation_specs(
    workflow_name: String,
    operation_specs: Vec<OperationSpec>,
    input_map: InputMap,
) -> Vec<OperationSpec> {
    let service_operation_names = input_map.get_workflow_services_operations_names(workflow_name);

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

fn build_lookup_map(
    operation_specs: Vec<OperationSpec>,
    mut variable_aliases: VariableAliases,
) -> BTreeMap<(String, String), ServiceCodeGenerationInfo> {
    let iter = operation_specs.iter();
    create_future_variable_names(iter.clone(), &mut variable_aliases);
    create_response_struct_names(iter.clone(), &mut variable_aliases);

    create_response_aliases(iter, &mut variable_aliases);

    // add dependencies
    // add created variable aliases to responses
    // create response struct names

    // add looked up mapped variable aliases from InputMap to requests

    todo!()
}

fn create_future_variable_names(
    iter: std::slice::Iter<'_, OperationSpec>,
    variable_aliases: &mut VariableAliases,
) -> Vec<String> {
    iter.clone()
        .map(|_| variable_aliases.create_alias())
        .collect()
}

fn create_response_struct_names(
    iter: std::slice::Iter<'_, OperationSpec>,
    variable_aliases: &mut VariableAliases,
) -> Vec<String> {
    iter.clone()
        .map(|_| variable_aliases.create_alias())
        .collect()
}

fn create_response_aliases(
    iter: std::slice::Iter<'_, OperationSpec>,
    variable_aliases: &mut VariableAliases,
) -> Vec<NestedNode<Variable>> {
    iter.clone()
        .map(|operation_spec| add_nested_response_aliases(operation_spec, variable_aliases))
        .collect()
}

fn add_nested_response_aliases(
    operation_spec: &OperationSpec,
    variable_aliases: &mut VariableAliases,
) -> NestedNode<Variable> {
    traverse_nested_type(
        operation_spec.response_specs.first().unwrap().body.clone(),
        |response_schema, (variable_aliases, alias_accumulator)| {
            if let Some(name) = response_schema.name {
                alias_accumulator.push(name);
            }

            let namespaced_name = alias_accumulator.join("/");

            let alias = (variable_aliases).create_stored_alias(namespaced_name);

            alias
        },
        |_, _, _| {},
        |schema| schema.properties,
        &mut (
            variable_aliases,
            vec![
                operation_spec.spec_name.to_string(),
                operation_spec.operation_id.to_string(),
            ],
        ),
    )
}
