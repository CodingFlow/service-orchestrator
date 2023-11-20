pub mod build_service_operation_lookup_map;
pub mod build_workflow_response_lookup_map;
mod create_response_aliases;
mod filter_to_used_operation_specs;
pub mod generate_response_variables;
mod get_service_urls;

use self::{
    build_service_operation_lookup_map::ServiceCodeGenerationInfo,
    build_workflow_response_lookup_map::WorkflowResponseCodeGenerationInfo,
};
use super::variables::VariableAliases;
use crate::{
    generate_workflows::input_map::InputMap,
    parse_specs::{get_operation_specs, SpecType},
};
use build_service_operation_lookup_map::build_service_operation_lookup_map;
use build_workflow_response_lookup_map::build_workflow_response_lookup_map;
use filter_to_used_operation_specs::filter_to_used_operation_specs;
use get_service_urls::get_service_urls;
use std::collections::BTreeMap;

pub struct ServiceCallGenerationInfo {
    pub service_calls: (
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
    pub workflow_service_response: WorkflowResponseCodeGenerationInfo,
}

pub fn build_service_call_view_data(
    workflow_name: String,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
) -> ServiceCallGenerationInfo {
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

    ServiceCallGenerationInfo {
        service_calls: generation_infos,
        workflow_service_response: workflow_response_info,
    }
}
