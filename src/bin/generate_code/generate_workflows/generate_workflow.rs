mod generate_workflow_request;
mod generate_workflow_response;

use http::Method;
use oas3::spec::{Operation, PathItem};

use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;

use crate::{
    extract_request_values_from_spec::extract_request_values_from_spec,
    generate_create_filter::WorkflowDefinitionNames, generate_re_exports::ReExports, SpecInfo,
};

use super::input_map::InputMap;

pub fn generate_workflow(
    path_item: &PathItem,
    operation: &Operation,
    spec_info: &SpecInfo,
    method: Method,
    path_string: &String,
    input_map: &InputMap,
    re_exports: &mut ReExports,
) -> WorkflowDefinitionNames {
    let request_values_from_spec =
        extract_request_values_from_spec(path_item, operation, &spec_info.spec);

    let (query_struct_name, request_module_name) = generate_workflow_request(
        method,
        path_string.to_string(),
        request_values_from_spec.clone(),
        spec_info.name.clone(),
        re_exports,
    );

    let response_module_name = generate_workflow_response(
        operation.responses.clone(),
        &spec_info,
        request_values_from_spec,
        query_struct_name,
        request_module_name.clone(),
        input_map,
        re_exports,
    );

    WorkflowDefinitionNames {
        request_name: request_module_name,
        response_name: response_module_name,
    }
}
