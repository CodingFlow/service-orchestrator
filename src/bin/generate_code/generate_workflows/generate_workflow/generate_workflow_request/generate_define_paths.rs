use codegen::Scope;

use crate::{
    generate_workflows::{
        add_variable_aliases_to_request_parameters::{
            RequestParameter, WorkflowPathPart, WorkflowVariable,
        },
        input_map::Variable,
    },
    parse_specs::parse_schema::to_string_schema,
};

use super::format_tuple;

pub fn generate_define_paths(scope: &mut Scope, path_parts: Vec<WorkflowPathPart>) {
    let formatted_parameters: Vec<String> = path_parts
        .to_vec()
        .iter()
        .filter(|path_part| (*path_part).schema_type.is_some())
        .map(|path_part| -> String {
            let name = match path_part.name.clone() {
                WorkflowVariable::Variable(name) => name,
                _ => Variable {
                    original_name: String::new(),
                    alias: String::new(),
                },
            };

            to_string_schema(
                path_part.schema_type.unwrap(),
                Some(name.original_name.to_string()),
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

    for path_part in path_parts {
        let formatted_path_part = match path_part.name {
            WorkflowVariable::Name(name) => format!(".and(warp::path(\"{}\"))", name),
            WorkflowVariable::Variable(name) => {
                format!(
                    ".and(warp::path::param::<{}>())",
                    to_string_schema(
                        path_part.schema_type.unwrap(),
                        Some(name.original_name.to_string())
                    )
                )
            }
        };

        function.line(formatted_path_part);
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
