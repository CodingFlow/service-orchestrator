mod generate_imports;
mod generate_map_response;
mod generate_response_structs;
mod generate_response_structure;

use codegen::Scope;

use generate_imports::generate_imports;
use generate_map_response::generate_map_response;
use generate_response_structure::generate_response_structure;

use crate::{
    generate_re_exports::{ReExports, ReExportsBehavior},
    generate_workflows::input_map::InputMap,
    parse_specs::ResponseSpec,
};

use super::{build_view_data::WorkflowRequestSpec, variables::VariableAliases};

pub fn generate_workflow_response(
    workflow_response_specs: Vec<ResponseSpec>,
    workflow_name: String,
    workflow_request_spec: WorkflowRequestSpec,
    request_module_name: String,
    input_map: &mut InputMap,
    re_exports: &mut ReExports,
    mut variable_aliases: VariableAliases,
) -> String {
    let mut scope = Scope::new();

    let WorkflowRequestSpec {
        query_struct_name, ..
    } = workflow_request_spec.clone();

    generate_imports(&mut scope, &query_struct_name, request_module_name);

    let status_code_struct_names = generate_response_structure(workflow_response_specs, &mut scope);

    generate_map_response(
        status_code_struct_names,
        &mut scope,
        workflow_request_spec,
        &query_struct_name,
        input_map,
        workflow_name.to_string(),
        &mut variable_aliases,
    );

    let module_name = format!("{}_workflow_response_definition", workflow_name);

    re_exports.add(module_name.clone(), scope.to_string());

    module_name
}
