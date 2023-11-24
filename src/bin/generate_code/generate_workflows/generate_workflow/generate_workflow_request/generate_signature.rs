use crate::{
    generate_workflows::generate_workflow::{
        build_service_call_view_data::generate_response_variables::ResponseAlias,
        build_workflow_request_view_data::{RequestParameter, WorkflowPathPart},
    },
    traversal::NestedNode,
};
use codegen::Function;

pub fn generate_signature(
    function: &mut Function,
    path_parameters: Vec<WorkflowPathPart>,
    query: Vec<RequestParameter>,
    query_struct_name: &str,
    body: Option<NestedNode<ResponseAlias>>,
) {
    let mut parameters: Vec<String> = path_parameters
        .iter()
        .filter(|path_part| (*path_part).alias.is_some())
        .map(|path_part| -> String { path_part.formatted_type.clone().unwrap() })
        .collect();

    if query.len() > 0 {
        parameters.push(query_struct_name.to_string());
    }

    if let Some(nested_request_alias) = body {
        parameters.push(nested_request_alias.current.variable_alias);
    }

    let formatted_parameters = parameters.join(",");

    function.vis("pub").ret(format!(
        "impl Filter<Extract = {}, Error = warp::Rejection> + Clone",
        format!("({})", formatted_parameters)
    ));
}
