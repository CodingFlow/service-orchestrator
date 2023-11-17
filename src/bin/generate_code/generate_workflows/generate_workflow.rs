mod add_variable_aliases_to_request_parameters;
mod generate_workflow_request;
mod generate_workflow_response;

use add_variable_aliases_to_request_parameters::add_variable_aliases_to_request_parameters;
use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;

use crate::{
    generate_create_filter::WorkflowDefinitionNames, generate_re_exports::ReExports,
    parse_specs::OperationSpec,
};

use super::input_map::InputMap;

pub fn generate_workflow(
    operation_spec: OperationSpec,
    input_map: &mut InputMap,
    re_exports: &mut ReExports,
) -> WorkflowDefinitionNames {
    let workflow_operation_spec =
        add_variable_aliases_to_request_parameters(operation_spec, input_map);

    let (query_struct_name, request_module_name) = generate_workflow_request(
        workflow_operation_spec.request_spec.clone(),
        workflow_operation_spec.operation_id.to_string(),
        re_exports,
    );

    let response_module_name = generate_workflow_response(
        workflow_operation_spec.response_spec,
        workflow_operation_spec.operation_id,
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
