mod create_function_signature;
mod create_query_destructure;
mod create_reply;
mod create_service_calls;

use crate::{
    generate_workflows::{
        extract_request_parameters_from_spec::RequestParameters, input_map::InputMap,
    },
    traversal::NestedNode,
};
use codegen::{Function, Scope};
use create_function_signature::create_function_signature;
use create_query_destructure::create_query_destructure;
use create_reply::create_reply;
use create_service_calls::create_service_calls;

use super::generate_response_structure::ResponseWithStructName;

pub fn generate_map_response(
    status_code_struct_names: Vec<(String, NestedNode<ResponseWithStructName>)>,
    scope: &mut Scope,
    request_parameters: RequestParameters,
    query_struct_name: &str,
    input_map: &InputMap,
    workflow_name: String,
) {
    let map_functions: Vec<Function> = status_code_struct_names
        .iter()
        .map(|status_code_struct_name_node| -> Function {
            map_function(
                status_code_struct_name_node.clone(),
                request_parameters.clone(),
                query_struct_name,
                input_map,
                workflow_name.to_string(),
            )
        })
        .collect();

    for function in map_functions {
        scope.push_fn(function);
    }
}

fn map_function(
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    request_parameters: RequestParameters,
    query_struct_name: &str,
    input_map: &InputMap,
    workflow_name: String,
) -> Function {
    let mut function = Function::new("map_response");

    create_function_signature(
        &mut function,
        request_parameters.path_parameters,
        query_struct_name,
    );

    create_query_destructure(
        &mut function,
        query_struct_name,
        request_parameters.query_parameters.to_vec(),
    );

    // create_service_calls(&mut function);

    create_reply(
        &mut function,
        status_code_struct_name_node,
        request_parameters.query_parameters,
        input_map,
        workflow_name,
    );

    function
}
