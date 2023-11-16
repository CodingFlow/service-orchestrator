mod generate_workflow_request;
mod generate_workflow_response;

use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;

use crate::{generate_create_filter::WorkflowDefinitionNames, generate_re_exports::ReExports};

use super::{
    add_variable_aliases_to_request_parameters::WorkflowOperationSpec, input_map::InputMap,
};

pub fn generate_workflow(
    workflow_operation_spec: WorkflowOperationSpec,
    input_map: &mut InputMap,
    re_exports: &mut ReExports,
) -> WorkflowDefinitionNames {
    let (query_struct_name, request_module_name) = generate_workflow_request(
        workflow_operation_spec.request_spec.clone(),
        workflow_operation_spec.spec_name.to_string(),
        re_exports,
    );

    let response_module_name = generate_workflow_response(
        workflow_operation_spec.response_spec,
        workflow_operation_spec.spec_name,
        workflow_operation_spec.request_spec,
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
