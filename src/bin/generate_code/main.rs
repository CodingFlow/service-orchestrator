mod extract_request_values_from_spec;
mod generate_re_exports;
mod generate_workflow;
pub mod spec_parsing;
pub mod traversal;

use generate_re_exports::{ReExports, ReExportsBehavior};
use generate_workflow::generate_workflow;
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
    let mut re_exports = ReExports::new();
    generate_workflow(
        path_item,
        operation,
        spec,
        method,
        path_string,
        &mut re_exports,
    );

    re_exports.generate();
}
