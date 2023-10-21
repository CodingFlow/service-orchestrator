use std::collections::HashMap;
use std::fs;

use codegen::Scope;
use oas3::{Schema, Spec};

use oas3::spec::Operation;

pub fn generate_workflow_response(operation: &Operation, spec: &Spec) {
    let mut scope = Scope::new();

    let response_values: HashMap<String, Schema> =
        extract_response_values_from_spec(operation, spec);

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn extract_response_values_from_spec(
    operation: &Operation,
    spec: &Spec,
) -> HashMap<String, Schema> {
    todo!()
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_response_definition.rs", code);
}
