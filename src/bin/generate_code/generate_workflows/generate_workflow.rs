mod generate_workflow_request;
mod generate_workflow_response;

use http::Method;
use oas3::spec::{Operation, SchemaType};

use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;

use crate::{
    generate_create_filter::WorkflowDefinitionNames, generate_re_exports::ReExports, SpecInfo,
};

use super::{extract_request_parameters_from_spec::RequestParameters, input_map::InputMap};

pub fn generate_workflow(
    request_parameters: RequestParameters,
    operation: &Operation,
    spec_info: &SpecInfo,
    method: Method,
    path_string: &String,
    input_map: &InputMap,
    re_exports: &mut ReExports,
) -> WorkflowDefinitionNames {
    let (query_struct_name, request_module_name) = generate_workflow_request(
        method,
        path_string.to_string(),
        request_parameters.clone(),
        spec_info.name.clone(),
        re_exports,
    );

    let response_module_name = generate_workflow_response(
        operation.responses.clone(),
        &spec_info,
        request_parameters,
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
