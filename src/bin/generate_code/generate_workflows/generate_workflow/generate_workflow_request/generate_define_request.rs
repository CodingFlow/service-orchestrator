use codegen::Scope;

use crate::generate_workflows::generate_workflow::build_request_view_data::WorkflowPathPart;

pub fn generate_define_request(
    scope: &mut Scope,
    path_parameters: Vec<WorkflowPathPart>,
    query_struct_name: &str,
) {
    let mut parameters: Vec<String> = path_parameters
        .iter()
        .filter(|path_part| (*path_part).alias.is_some())
        .map(|path_part| -> String { path_part.formatted_type.clone().unwrap() })
        .collect();

    parameters.push(query_struct_name.to_string());

    let formatted_parameters = parameters.join(",");

    scope
        .new_fn("define_request")
        .vis("pub")
        .ret(format!(
            "impl Filter<Extract = {}, Error = warp::Rejection> + Clone",
            format!("({})", formatted_parameters)
        ))
        .line("let http_method = define_method();")
        .line("let with_paths = define_paths(http_method);")
        .line("let with_query = define_query(with_paths);")
        .line("with_query");
}
