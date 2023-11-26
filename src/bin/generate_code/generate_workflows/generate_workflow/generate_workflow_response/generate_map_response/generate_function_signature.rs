use codegen::Function;

use crate::{
    generate_workflows::generate_workflow::{
        build_service_call_view_data::generate_body_variables::BodyPropertyAlias,
        build_workflow_request_view_data::{QueryVariables, RequestParameter, WorkflowPathPart},
    },
    traversal::NestedNode,
};

pub fn generate_function_signature(
    function: &mut Function,
    path_parts: Vec<WorkflowPathPart>,
    query: Vec<RequestParameter>,
    query_variables: QueryVariables,
    request_body: Option<NestedNode<BodyPropertyAlias>>,
    request_body_local_variable: String,
) {
    function.vis("pub");
    function.set_async(true);

    create_function_arguments(
        path_parts,
        function,
        query,
        query_variables,
        request_body,
        request_body_local_variable,
    );

    function.ret("Result<impl warp::Reply, warp::Rejection>");
}

fn create_function_arguments(
    path_parts: Vec<WorkflowPathPart>,
    function: &mut Function,
    query: Vec<RequestParameter>,
    query_variables: QueryVariables,
    request_body: Option<NestedNode<BodyPropertyAlias>>,
    request_body_local_variable: String,
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

    if query.len() > 0 {
        function.arg(&query_variables.local_variable, query_variables.struct_name);
    }

    if let Some(nested_response_alias) = request_body {
        function.arg(
            &request_body_local_variable,
            nested_response_alias.current.variable_alias,
        );
    };
}
