use codegen::Function;

use crate::{
    generate_workflows::generate_workflow::{
        build_service_call_view_data::generate_response_variables::ResponseAlias,
        build_workflow_request_view_data::{RequestParameter, WorkflowPathPart},
    },
    traversal::NestedNode,
};

pub fn generate_function_signature(
    function: &mut Function,
    path_parts: Vec<WorkflowPathPart>,
    query: Vec<RequestParameter>,
    query_struct_name: &str,
    request_body: Option<NestedNode<ResponseAlias>>,
) {
    function.vis("pub");
    function.set_async(true);

    create_function_arguments(path_parts, function, query, query_struct_name, request_body);

    function.ret("Result<impl warp::Reply, warp::Rejection>");
}

fn create_function_arguments(
    path_parts: Vec<WorkflowPathPart>,
    function: &mut Function,
    query: Vec<RequestParameter>,
    query_struct_name: &str,
    request_body: Option<NestedNode<ResponseAlias>>,
) {
    let path_parameters_info: Vec<(String, String)> = path_parts
        .iter()
        .filter(|path_part| (*path_part).alias.is_some())
        .map(|path_part| -> (String, String) {
            (
                path_part.alias.clone().unwrap(),
                path_part.formatted_type.clone().unwrap(),
            )
        })
        .collect();

    for (name, schema_type) in path_parameters_info {
        function.arg(&name, schema_type);
    }

    // TODO: generate aliases and pass them in for these two parameters.
    if query.len() > 0 {
        function.arg("parameters", query_struct_name);
    }

    if let Some(nested_response_alias) = request_body {
        function.arg("body", nested_response_alias.current.variable_alias);
    };
}
