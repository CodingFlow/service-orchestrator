use codegen::Function;

use crate::generate_workflows::extract_request_parameters_from_spec::RequestParameter;

pub fn create_query_destructure(
    function: &mut Function,
    query_struct_name: &str,
    query_parameters: Vec<RequestParameter>,
) {
    function.line(format_query_destructure(
        query_struct_name,
        query_parameters,
    ));
}

fn format_query_destructure(
    query_struct_name: &str,
    query_parameters: Vec<RequestParameter>,
) -> String {
    let variables: Vec<String> = query_parameters
        .iter()
        .map(|parameter| -> String {
            format!(
                "{}: mut {}",
                parameter.name.original_name, parameter.name.alias
            )
        })
        .collect();

    format!(
        "let {} {{ {} }} = parameters;",
        query_struct_name,
        variables.join(",")
    )
}
