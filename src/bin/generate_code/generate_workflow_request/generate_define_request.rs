use codegen::Scope;
use oas3::spec::SchemaType;

use crate::spec_parsing::to_string_schema_type_primitive;

pub fn generate_define_request(
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
) {
    let mut parameters = path_parameters
        .iter()
        .map(|(_, schema_type)| -> &str { to_string_schema_type_primitive(*schema_type) })
        .collect::<Vec<&str>>();

    parameters.push(query_struct_name);

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
