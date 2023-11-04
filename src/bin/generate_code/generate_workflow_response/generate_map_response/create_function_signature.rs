use codegen::Function;
use oas3::spec::SchemaType;

use crate::spec_parsing::to_string_schema;

pub fn create_function_signature(
    function: &mut Function,
    path_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
) {
    function.vis("pub");
    function.set_async(true);

    create_function_arguments(path_parameters, function, query_struct_name);

    function.ret("Result<impl warp::Reply, warp::Rejection>");
}

fn create_function_arguments(
    path_parameters: Vec<(String, SchemaType)>,
    function: &mut Function,
    query_struct_name: &str,
) {
    let path_parameters_info: Vec<(&str, String)> = path_parameters
        .iter()
        .map(|(name, schema_type)| -> (&str, String) {
            (name, to_string_schema(*schema_type, None))
        })
        .collect();

    for (name, schema_type) in path_parameters_info {
        function.arg(name, schema_type);
    }

    function.arg("parameters", query_struct_name);
}
