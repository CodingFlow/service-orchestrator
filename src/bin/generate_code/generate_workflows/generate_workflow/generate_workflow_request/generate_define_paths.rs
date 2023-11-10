use codegen::Scope;

use crate::{
    generate_workflows::extract_request_parameters_from_spec::RequestParameter,
    spec_parsing::to_string_schema,
};

use super::format_tuple;

pub fn generate_define_paths(
    scope: &mut Scope,
    path_string: String,
    path_parameters: Vec<RequestParameter>,
) {
    let formatted_parameters: Vec<String> = path_parameters
        .to_vec()
        .iter()
        .map(|parameter| -> String {
            to_string_schema(
                parameter.schema_type,
                Some(parameter.name.original_name.to_string()),
            )
        })
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

    let path_parts = path_string.split('/');

    for path_part in path_parts {
        match (path_part.get(..1), path_part.chars().rev().nth(0)) {
            (Some("{"), Some('}')) => function.line(format!(
                ".and(warp::path::param::<{}>())",
                get_path_parameter(path_parameters.to_vec(), path_part)
            )),
            (Some(_), Some(_)) => function.line(format!(".and(warp::path(\"{}\"))", path_part)),
            _ => function,
        };
    }

    function.line(".and(warp::path::end())");
}

fn get_path_parameter(path_parameters: Vec<RequestParameter>, path_part: &str) -> String {
    let request_parameter = path_parameters
        .iter()
        .find(|parameter| -> bool {
            parameter.name.original_name == remove_first_and_last(path_part)
        })
        .unwrap();

    to_string_schema(
        request_parameter.schema_type,
        Some(request_parameter.name.original_name.to_string()),
    )
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
