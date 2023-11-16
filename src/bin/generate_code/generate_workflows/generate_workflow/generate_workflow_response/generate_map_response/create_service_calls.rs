mod build_loopkup_map;
mod generate_response_structs;
mod generate_service_calls;
mod variables;

use build_loopkup_map::build_lookup_map;
use codegen::{Function, Scope};
use generate_response_structs::generate_response_structs;
use generate_service_calls::generate_service_calls;

use crate::{
    generate_workflows::input_map::{InputMap, InputMapBehavior},
    parse_specs::{get_operation_specs, OperationSpec, SpecType},
};

use self::variables::VariableAliases;

pub fn create_service_calls(
    function: &mut Function,
    input_map: &mut InputMap,
    workflow_name: String,
    scope: &mut Scope,
) {
    let operation_specs = get_operation_specs(SpecType::Service);

    let used_operation_specs =
        filter_to_used_operation_specs(workflow_name.to_string(), operation_specs, &input_map);

    let mut variable_aliases = VariableAliases::new();
    let response_struct_names =
        create_response_struct_names(used_operation_specs.iter(), &mut variable_aliases);

    generate_response_structs(
        scope,
        used_operation_specs.clone(),
        response_struct_names.to_vec(),
    );

    let generation_infos = build_lookup_map(
        used_operation_specs,
        response_struct_names,
        &mut variable_aliases,
        workflow_name,
        input_map,
    );

    generate_service_calls(scope, function, generation_infos, variable_aliases);
}

fn create_response_struct_names(
    iter: std::slice::Iter<'_, OperationSpec>,
    variable_aliases: &mut VariableAliases,
) -> Vec<String> {
    iter.clone()
        .map(|_| variable_aliases.create_alias())
        .collect()
}

fn filter_to_used_operation_specs(
    workflow_name: String,
    operation_specs: Vec<OperationSpec>,
    input_map: &InputMap,
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
