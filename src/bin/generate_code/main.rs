mod extract_request_values_from_spec;
mod generate_workflow_request;
mod generate_workflow_response;
pub mod spec_parsing;
pub mod traversal;

use extract_request_values_from_spec::extract_request_values_from_spec;
use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;
use http::Method;
use oas3::{
    spec::{Operation, PathItem},
    Spec,
};

fn main() {
    let spec = parse_config();

    for (path_string, path_item) in &spec.paths {
        for method in path_item.methods() {
            generate_code(&path_string, &path_item, method, &spec);
        }
    }
}

fn parse_config() -> oas3::Spec {
    match oas3::from_path("./src/workflow_spec.yaml") {
        Ok(spec) => spec,
        Err(_) => panic!("unable to read open API spec file"),
    }
}

fn generate_code(
    path_string: &String,
    path_item: &PathItem,
    (method, operation): (Method, &Operation),
    spec: &Spec,
) {
    let request_values_from_spec = extract_request_values_from_spec(path_item, operation, spec);
    let query_struct_name = generate_workflow_request(
        method,
        path_string.to_string(),
        request_values_from_spec.clone(),
    );
    generate_workflow_response(
        operation.responses.clone(),
        spec,
        request_values_from_spec,
        query_struct_name,
    );
}
