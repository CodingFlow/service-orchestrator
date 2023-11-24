mod generate_body;
mod generate_define_method;
mod generate_define_paths;
mod generate_define_query;
mod generate_query_struct;
mod generate_request_body_struct;
mod generate_signature;

use super::build_workflow_request_view_data::WorkflowRequestSpec;
use crate::generate_re_exports::{ReExports, ReExportsBehavior};
use codegen::Scope;
use generate_body::generate_body;
use generate_define_method::generate_method;
use generate_define_paths::generate_define_paths;
use generate_define_query::generate_define_query;
use generate_query_struct::generate_query_struct;
use generate_request_body_struct::generate_request_body_struct;
use generate_signature::generate_signature;

pub fn generate_workflow_request<'a>(
    workflow_request_spec: WorkflowRequestSpec,
    workflow_name: String,
    re_exports: &mut ReExports,
) -> String {
    let mut scope = Scope::new();

    scope.import("warp", "Filter");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");

    let WorkflowRequestSpec {
        method,
        query,
        path,
        query_struct_name,
        body,
    } = workflow_request_spec;

    generate_query_struct(&mut scope, query.to_vec(), &query_struct_name);
    generate_request_body_struct(&mut scope, body.clone());

    let mut function = scope.new_fn("define_request");

    generate_signature(
        &mut function,
        path.to_vec(),
        query.to_vec(),
        &query_struct_name,
        body.clone(),
    );
    generate_method(&mut function, method);
    generate_define_paths(&mut function, path.to_vec());
    generate_define_query(&mut function, query, &query_struct_name);
    generate_body(function, body);

    let module_name = format!("{}_workflow_request_definition", workflow_name);

    re_exports.add(module_name.clone(), scope.to_string());

    module_name
}
