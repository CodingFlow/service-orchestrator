use codegen::Scope;
use oas3::spec::SchemaType;

use crate::spec_parsing::to_string_schema;

pub fn generate_define_request(
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
) {
    let mut parameters: Vec<String> = path_parameters
        .iter()
        .map(|(name, schema_type)| -> String {
            to_string_schema(*schema_type, Some(name.to_string()))
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
