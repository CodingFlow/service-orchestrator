mod create_input_map;
mod generate_imports;
mod generate_map_response;
mod generate_response_structure;
mod parse_responses;

use std::collections::BTreeMap;
use std::fs;

use codegen::Scope;
use oas3::Spec;

use create_input_map::create_input_map;
use generate_imports::generate_imports;
use generate_map_response::generate_map_response;
use generate_response_structure::generate_response_structure;
use oas3::spec::{ObjectOrReference, Response, SchemaType};
use parse_responses::parse_responses;

pub fn generate_workflow_response(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
    (path_parameters, query_parameters): (Vec<(String, SchemaType)>, Vec<(String, SchemaType)>),
    query_struct_name: &str,
) {
    let mut scope = Scope::new();

    generate_imports(&mut scope, query_struct_name);

    let response_values = parse_responses(responses, spec);

    let status_code_struct_name_pairs =
        generate_response_structure(response_values.to_vec(), &mut scope);

    let input_map = create_input_map();

    generate_map_response(
        status_code_struct_name_pairs,
        &mut scope,
        path_parameters,
        query_parameters,
        query_struct_name,
        response_values,
        input_map.clone(),
    );

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_response_definition.rs", code);
}
