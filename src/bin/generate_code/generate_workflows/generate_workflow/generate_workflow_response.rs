mod generate_imports;
mod generate_map_response;
mod generate_response_structs;

use super::{
    build_service_call_view_data::{
        generate_response_variables::ResponseAlias, ServiceCallGenerationInfo,
    },
    build_workflow_request_view_data::WorkflowRequestSpec,
    build_workflow_response_view_data::WorkflowResponseGenerationInfo,
    variables::VariableAliases,
};
use crate::{
    generate_re_exports::{ReExports, ReExportsBehavior},
    traversal::NestedNode,
};
use codegen::Scope;
use generate_imports::generate_imports;
use generate_map_response::generate_map_response;

pub fn generate_workflow_response(
    workflow_name: String,
    workflow_request_spec: WorkflowRequestSpec,
    request_module_name: String,
    re_exports: &mut ReExports,
    mut variable_aliases: VariableAliases,
    service_call_view_data: ServiceCallGenerationInfo,
    workflow_response_generation_info: WorkflowResponseGenerationInfo,
) -> String {
    let mut scope = Scope::new();

    let WorkflowRequestSpec {
        query_struct_name, ..
    } = workflow_request_spec.clone();

    generate_imports(&mut scope, &query_struct_name, request_module_name);

    generate_map_response(
        &mut scope,
        workflow_request_spec,
        &query_struct_name,
        &mut variable_aliases,
        service_call_view_data,
        workflow_response_generation_info,
    );

    let module_name = format!("{}_workflow_response_definition", workflow_name);

    re_exports.add(module_name.clone(), scope.to_string());

    module_name
}
