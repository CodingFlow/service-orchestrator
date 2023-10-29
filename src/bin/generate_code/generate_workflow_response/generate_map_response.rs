use codegen::{Function, Scope};
use oas3::spec::SchemaType;

use crate::spec_parsing::{to_string_schema_type_primitive, ParsedSchema};

pub fn generate_map_response(
    status_code_struct_name_pairs: Vec<(String, String)>,
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
    response_values: Vec<(String, ParsedSchema)>,
) {
    let map_functions: Vec<Function> = status_code_struct_name_pairs
        .iter()
        .map(|(status_code, struct_name)| -> Function {
            let mut function = Function::new("map_response");

            function.vis("pub");

            let path_parameters_info: Vec<(&str, &str)> = path_parameters
                .iter()
                .map(|(name, schema_type)| -> (&str, &str) {
                    (name, to_string_schema_type_primitive(*schema_type))
                })
                .collect();

            for (name, schema_type) in path_parameters_info {
                function.arg(name, schema_type);
            }

            function.arg("parameters", query_struct_name);

            function.ret("Json");

            function.line(format!("reply::json(&{} {{", struct_name));

            let parsed_schema = &response_values
                .iter()
                .find(|(parsed_schema_status_code, _)| -> bool {
                    status_code == parsed_schema_status_code
                })
                .unwrap()
                .1;

            for response_property in parsed_schema.properties.clone().unwrap() {
                function.line(format!("{}:{},", response_property.name.unwrap(), "1.0"));
            }

            function.line("})");

            function
        })
        .collect();

    for function in map_functions {
        scope.push_fn(function);
    }
}
