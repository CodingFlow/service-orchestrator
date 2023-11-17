use codegen::Scope;

use crate::generate_workflows::generate_workflow::add_variable_aliases_to_request_parameters::WorkflowPathPart;

use super::format_tuple;

pub fn generate_define_paths(scope: &mut Scope, path_parts: Vec<WorkflowPathPart>) {
    let formatted_parameters: Vec<String> = path_parts
        .to_vec()
        .iter()
        .filter(|path_part| (*path_part).alias.is_some())
        .map(|path_part| -> String { path_part.formatted_type.clone().unwrap() })
        .collect();

    let function = scope
        .new_fn("define_paths")
        .arg(
            "http_method",
            "impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy",
        )
        .ret(format!(
            "impl Filter<Extract = {}, Error = warp::reject::Rejection> + Copy",
            format_tuple(formatted_parameters)
        ))
        .line("http_method");

    for path_part in path_parts {
        let formatted_path_part = match path_part.formatted_type {
            Some(formatted_type) => format!(".and(warp::path::param::<{}>())", formatted_type),
            None => format!(".and(warp::path(\"{}\"))", path_part.name),
        };

        function.line(formatted_path_part);
    }

    function.line(".and(warp::path::end())");
}
