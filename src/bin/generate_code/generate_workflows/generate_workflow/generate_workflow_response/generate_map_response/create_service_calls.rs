mod build_service_operation_lookup_map;
mod build_workflow_response_lookup_map;
mod filter_to_used_operation_specs;
mod generate_service_calls;
mod generate_service_response_structs;
mod generate_stream_enum;
mod get_service_urls;

use crate::{
    generate_workflows::{generate_workflow::variables::VariableAliases, input_map::InputMap},
    parse_specs::{get_operation_specs, SpecType},
};
use build_service_operation_lookup_map::build_service_operation_lookup_map;
use build_workflow_response_lookup_map::build_workflow_response_lookup_map;
use codegen::{Function, Scope};
use filter_to_used_operation_specs::filter_to_used_operation_specs;
use generate_service_calls::generate_service_calls;
use generate_service_response_structs::generate_service_response_structs;
use generate_stream_enum::generate_stream_enum;
use get_service_urls::get_service_urls;

pub fn create_service_calls(
    function: &mut Function,
    input_map: &mut InputMap,
    workflow_name: String,
    scope: &mut Scope,
    variable_aliases: &mut VariableAliases,
) {
    let service_urls = get_service_urls();
    let operation_specs = get_operation_specs(SpecType::Service);

    let used_operation_specs =
        filter_to_used_operation_specs(workflow_name.to_string(), operation_specs, &input_map);

    let generation_infos = build_service_operation_lookup_map(
        used_operation_specs,
        variable_aliases,
        workflow_name.to_string(),
        service_urls,
        input_map,
    );

    let workflow_response_info = build_workflow_response_lookup_map(
        generation_infos.0.clone(),
        variable_aliases,
        workflow_name,
        input_map,
    );

    generate_service_response_structs(scope, generation_infos.1.to_vec());

    generate_stream_enum(scope, generation_infos.clone());

    generate_service_calls(
        scope,
        function,
        generation_infos,
        workflow_response_info,
        variable_aliases,
    );
}
