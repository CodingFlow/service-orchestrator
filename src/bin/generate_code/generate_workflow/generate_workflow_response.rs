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

use crate::generate_re_exports::{ReExports, ReExportsBehavior};

pub fn generate_workflow_response(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
    (path_parameters, query_parameters): (Vec<(String, SchemaType)>, Vec<(String, SchemaType)>),
    query_struct_name: &str,
    re_exports: &mut ReExports,
) {
    let mut scope = Scope::new();

    generate_imports(&mut scope, query_struct_name);

    let parsed_spec_responses = parse_responses(responses, spec);

    let status_code_struct_names =
        generate_response_structure(parsed_spec_responses.to_vec(), &mut scope);

    let input_map = create_input_map();

    generate_map_response(
        status_code_struct_names,
        &mut scope,
        path_parameters,
        query_parameters,
        query_struct_name,
        input_map.clone(),
    );

    println!("{}", scope.to_string());

    re_exports.add(
        "workflow_response_definition".to_string(),
        scope.to_string(),
    );
}
