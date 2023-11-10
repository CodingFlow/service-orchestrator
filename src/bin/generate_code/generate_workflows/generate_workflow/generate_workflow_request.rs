mod generate_define_method;
mod generate_define_paths;
mod generate_define_query;
mod generate_define_request;
mod generate_query_struct;

use codegen::Scope;
use generate_define_method::generate_define_method;
use generate_define_paths::generate_define_paths;
use generate_define_query::generate_define_query;
use generate_define_request::generate_define_request;
use generate_query_struct::generate_query_struct;
use http::Method;

use crate::{
    generate_re_exports::{ReExports, ReExportsBehavior},
    generate_workflows::extract_request_parameters_from_spec::RequestParameters,
};

pub fn generate_workflow_request<'a>(
    method: Method,
    path_string: String,
    request_parameters: RequestParameters,
    workflow_name: String,
    re_exports: &mut ReExports,
) -> (&'a str, String) {
    let mut scope = Scope::new();

    scope.import("warp::reject", "Rejection");
    scope.import("warp", "Filter");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");

    let RequestParameters {
        path_parameters,
        query_parameters,
    } = request_parameters;

    let query_struct_name = generate_query_struct(&mut scope, query_parameters);

    generate_define_request(&mut scope, path_parameters.to_vec(), query_struct_name);
    generate_define_method(&mut scope, method);
    generate_define_paths(&mut scope, path_string, path_parameters.to_vec());
    generate_define_query(&mut scope, path_parameters, query_struct_name);

    let module_name = format!("{}_workflow_request_definition", workflow_name);

    re_exports.add(module_name.clone(), scope.to_string());

    (query_struct_name, module_name)
}

fn format_tuple(input: Vec<String>) -> String {
    match input.len() {
        1 => format!("({},)", input.join(",")),
        _ => format!("({})", input.join(",")),
    }
}
