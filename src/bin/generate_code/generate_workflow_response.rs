mod generate_imports;
mod generate_map_response;
mod generate_response_structure;
mod parse_responses;

use std::collections::HashMap;
use std::fs;
use std::{collections::BTreeMap, path};

use codegen::Scope;
use oas3::Spec;

use generate_imports::generate_imports;
use generate_map_response::generate_map_response;
use generate_response_structure::generate_response_structure;
use oas3::spec::{ObjectOrReference, Response, SchemaType};
use parse_responses::parse_responses;
use serde_json::{Map, Value};

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
        input_map,
    );

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn create_input_map() -> Map<String, Value> {
    let file = match fs::File::open("./src/workflow_mapping.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read workflow mapping configuration file."),
    };
    let config: serde_json::Value = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse workflow mapping configuration file."),
    };

    let workflow_config = config.get("Workflow A").unwrap();
    let response = workflow_config
        .get("response")
        .unwrap()
        .as_object()
        .unwrap();

    response.clone()
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_response_definition.rs", code);
}
