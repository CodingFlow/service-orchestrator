use codegen::Scope;

use crate::{
    generate_workflows::extract_request_parameters_from_spec::RequestParameter,
    spec_parsing::to_string_schema,
};

pub fn generate_define_request(
    scope: &mut Scope,
    path_parameters: Vec<RequestParameter>,
    query_struct_name: &str,
) {
    let mut parameters: Vec<String> = path_parameters
        .iter()
        .map(|parameter| -> String {
            to_string_schema(
                parameter.schema_type,
                Some(parameter.name.original_name.to_string()),
            )
        })
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
