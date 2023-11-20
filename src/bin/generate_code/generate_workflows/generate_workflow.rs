mod build_view_data;
mod generate_workflow_request;
mod generate_workflow_response;
mod variables;

use build_view_data::build_view_data;
use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;

use crate::{
    generate_create_filter::WorkflowDefinitionNames, generate_re_exports::ReExports,
    parse_specs::OperationSpec,
};

use self::variables::VariableAliases;

use super::input_map::InputMap;

pub fn generate_workflow(
    operation_spec: OperationSpec,
    input_map: &mut InputMap,
    re_exports: &mut ReExports,
) -> WorkflowDefinitionNames {
    let mut variable_aliases = VariableAliases::new();
    let workflow_operation_spec =
        build_view_data(operation_spec.clone(), input_map, &mut variable_aliases);

    let request_module_name = generate_workflow_request(
        workflow_operation_spec.request_spec.clone(),
        workflow_operation_spec.operation_id.to_string(),
        re_exports,
    );

    let response_module_name = generate_workflow_response(
        operation_spec,
        workflow_operation_spec.operation_id,
        workflow_operation_spec.request_spec,
        request_module_name.clone(),
        input_map,
        re_exports,
        variable_aliases,
    );

    WorkflowDefinitionNames {
        request_name: request_module_name,
        response_name: response_module_name,
    }
}
