mod create_input_map;
mod generate_imports;
mod generate_map_response;
mod generate_response_structure;
mod parse_responses;

use std::collections::BTreeMap;

use codegen::Scope;

use create_input_map::create_input_map;
use generate_imports::generate_imports;
use generate_map_response::generate_map_response;
use generate_response_structure::generate_response_structure;
use oas3::spec::{ObjectOrReference, Response, SchemaType};
use parse_responses::parse_responses;

use crate::{
    generate_re_exports::{ReExports, ReExportsBehavior},
    SpecInfo,
};

pub fn generate_workflow_response(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec_info: &SpecInfo,
    (path_parameters, query_parameters): (Vec<(String, SchemaType)>, Vec<(String, SchemaType)>),
    query_struct_name: &str,
    request_module_name: String,
    re_exports: &mut ReExports,
) -> String {
    let mut scope = Scope::new();

    generate_imports(&mut scope, query_struct_name, request_module_name);

    let parsed_spec_responses = parse_responses(responses, &spec_info.spec);

    let status_code_struct_names =
        generate_response_structure(parsed_spec_responses.to_vec(), &mut scope);

    let input_map = create_input_map(spec_info.name.clone());

    generate_map_response(
        status_code_struct_names,
        &mut scope,
        path_parameters,
        query_parameters,
        query_struct_name,
        input_map.clone(),
    );

    let module_name = format!("{}_workflow_response_definition", spec_info.name);

    re_exports.add(module_name.clone(), scope.to_string());

    module_name
}
