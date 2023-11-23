mod build_service_call_view_data;
mod build_workflow_request_view_data;
mod build_workflow_response_view_data;
mod create_request_aliases;
mod create_response_aliases;
mod create_workflow_response_aliases;
mod generate_workflow_request;
mod generate_workflow_response;
mod variables;

use self::variables::VariableAliases;
use super::input_map::InputMap;
use crate::{
    generate_create_filter::WorkflowDefinitionNames, generate_re_exports::ReExports,
    parse_specs::OperationSpec,
};
use build_service_call_view_data::build_service_call_view_data;
use build_workflow_request_view_data::build_workflow_request_view_data;
use build_workflow_response_view_data::build_workflow_response_view_data;
use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;
use std::collections::BTreeMap;
use url::Url;

pub fn generate_workflow(
    operation_spec: OperationSpec,
    service_operation_specs: Vec<OperationSpec>,
    service_urls: BTreeMap<String, Url>,
    input_map: &mut InputMap,
    re_exports: &mut ReExports,
) -> WorkflowDefinitionNames {
    let mut variable_aliases = VariableAliases::new();

    let request_spec =
        build_workflow_request_view_data(operation_spec.clone(), input_map, &mut variable_aliases);

    let service_call_view_data = build_service_call_view_data(
        service_operation_specs,
        service_urls,
        operation_spec.operation_id.to_string(),
        input_map,
        &mut variable_aliases,
    );

    let response_aliases =
        build_workflow_response_view_data(&operation_spec, input_map, &mut variable_aliases);

    let request_module_name = generate_workflow_request(
        request_spec.clone(),
        operation_spec.operation_id.to_string(),
        re_exports,
    );

    let response_module_name = generate_workflow_response(
        operation_spec.operation_id,
        request_spec,
        request_module_name.clone(),
        re_exports,
        variable_aliases,
        service_call_view_data,
        response_aliases,
    );

    WorkflowDefinitionNames {
        request_name: request_module_name,
        response_name: response_module_name,
    }
}
