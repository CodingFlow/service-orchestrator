mod extract_request_values_from_spec;
mod generate_define_method;
mod generate_define_paths;
mod generate_define_query;
mod generate_define_request;
mod generate_query_struct;

use std::fs::{self};

use codegen::Scope;
use extract_request_values_from_spec::extract_request_values_from_spec;
use generate_define_method::generate_define_method;
use generate_define_paths::generate_define_paths;
use generate_define_query::generate_define_query;
use generate_define_request::generate_define_request;
use generate_query_struct::generate_query_struct;
use http::Method;
use oas3::{
    spec::{Operation, PathItem},
    Spec,
};

pub fn generate_workflow_request(
    path_item: &PathItem,
    operation: &Operation,
    spec: &Spec,
    method: Method,
    path_string: &String,
) {
    let mut scope = Scope::new();

    scope.import("warp::reject", "Rejection");
    scope.import("warp", "Filter");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");

    let (path_parameters, query_parameters) =
        extract_request_values_from_spec(path_item, operation, spec);

    let query_struct_name = generate_query_struct(&mut scope, query_parameters);

    generate_define_request(&mut scope, path_parameters.to_vec(), query_struct_name);
    generate_define_method(&mut scope, method);
    generate_define_paths(&mut scope, path_string, path_parameters.to_vec());
    generate_define_query(&mut scope, path_parameters, query_struct_name);

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn format_tuple(input: Vec<&str>) -> String {
    match input.len() {
        1 => format!("({},)", input.join(",")),
        _ => format!("({})", input.join(",")),
    }
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_request_definition.rs", code);
}
