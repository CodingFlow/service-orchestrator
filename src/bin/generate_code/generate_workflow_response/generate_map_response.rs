mod create_function_signature;
mod create_query_destructure;
mod create_reply;

use crate::{spec_parsing::ParsedSchema, traversal::NestedNode};
use codegen::{Function, Scope};
use create_function_signature::create_function_signature;
use create_query_destructure::create_query_destructure;
use create_reply::create_reply;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

pub fn generate_map_response(
    status_code_struct_names: Vec<(String, NestedNode<Option<String>>)>,
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
    parsed_spec_responses: Vec<(String, ParsedSchema)>,
    input_map: Map<String, Value>,
) {
    let map_functions: Vec<Function> = status_code_struct_names
        .iter()
        .map(|status_code_struct_name_node| -> Function {
            map_function(
                status_code_struct_name_node.clone(),
                path_parameters.to_vec(),
                query_parameters.to_vec(),
                query_struct_name,
                parsed_spec_responses.to_vec(),
                input_map.clone(),
            )
        })
        .collect();

    for function in map_functions {
        scope.push_fn(function);
    }
}

fn map_function(
    status_code_struct_name_node: (String, NestedNode<Option<String>>),
    path_parameters: Vec<(String, SchemaType)>,
    query_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
    parsed_spec_responses: Vec<(String, ParsedSchema)>,
    input_map: Map<String, Value>,
) -> Function {
    let mut function = Function::new("map_response");

    create_function_signature(&mut function, path_parameters, query_struct_name);

    create_query_destructure(&mut function, query_struct_name, &query_parameters);

    create_reply(
        &mut function,
        status_code_struct_name_node,
        parsed_spec_responses,
        input_map,
        query_parameters,
    );

    function
}
