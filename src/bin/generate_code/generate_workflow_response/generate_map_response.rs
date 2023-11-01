use codegen::{Function, Scope};
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::{
    spec_parsing::{to_string_schema, ParsedSchema},
    traversal::NestedNode,
};

pub fn generate_map_response(
    status_code_struct_name_pairs: Vec<NestedNode<(String, String)>>,
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
    response_values: Vec<(String, ParsedSchema)>,
    input_map: Map<String, Value>,
) {
    let map_functions: Vec<Function> = status_code_struct_name_pairs
        .iter()
        .map(|node| -> Function {
            map_function(
                node.clone(),
                path_parameters.to_vec(),
                query_parameters.to_vec(),
                query_struct_name,
                response_values.to_vec(),
                input_map.clone(),
            )
        })
        .collect();

    for function in map_functions {
        scope.push_fn(function);
    }
}

fn map_function(
    node: NestedNode<(String, String)>,
    path_parameters: Vec<(String, SchemaType)>,
    query_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
    response_values: Vec<(String, ParsedSchema)>,
    input_map: Map<String, Value>,
) -> Function {
    let (status_code, struct_name) = node.current.clone();

    let mut function = Function::new("map_response");

    function.vis("pub");

    let path_parameters_info: Vec<(&str, String)> = path_parameters
        .iter()
        .map(|(name, schema_type)| -> (&str, String) {
            (name, to_string_schema(*schema_type, None))
        })
        .collect();

    for (name, schema_type) in path_parameters_info {
        function.arg(name, schema_type);
    }

    function
        .arg("parameters", query_struct_name)
        .ret("Json")
        .line(format_query_destructure(
            query_struct_name,
            query_parameters.clone(),
        ))
        .line(format!("reply::json(&{} {{", struct_name));

    let parsed_schema = &response_values
        .iter()
        .find(|(parsed_schema_status_code, _)| -> bool {
            status_code == parsed_schema_status_code.to_string()
        })
        .unwrap()
        .1;

    for response_property in parsed_schema.properties.clone().unwrap() {
        function.line(format_response_fields(
            response_property,
            &input_map,
            query_parameters.to_vec(),
        ));
    }

    function.line("})");

    function
}

fn format_query_destructure(
    query_struct_name: &str,
    query_parameters: Vec<(String, SchemaType)>,
) -> String {
    let variables: Vec<String> = query_parameters
        .iter()
        .map(|(name, _)| -> String { format!("mut {}", name) })
        .collect();

    format!(
        "let {} {{ {} }} = parameters;",
        query_struct_name,
        variables.join(",")
    )
}

fn format_response_fields(
    response_property: ParsedSchema,
    input_map: &Map<String, Value>,
    query_parameters: Vec<(String, SchemaType)>,
) -> String {
    let property_name = response_property.name.unwrap();
    let mapped_value_name = input_map.get(&property_name).unwrap().as_str().unwrap();

    format!(
        "{}:{},",
        property_name,
        format_response_field_value(
            response_property.schema_type,
            query_parameters,
            mapped_value_name
        ),
    )
}

fn format_response_field_value(
    response_property_schema_type: SchemaType,
    query_parameters: Vec<(String, SchemaType)>,
    mapped_value_name: &str,
) -> String {
    match query_parameters
        .iter()
        .any(|(name, _)| -> bool { name.to_string() == mapped_value_name })
    {
        true => format!(
            "*{}.get_or_insert({})",
            mapped_value_name,
            convert_type_to_default_value(response_property_schema_type)
        ),
        false => mapped_value_name.to_string(),
    }
}

fn convert_type_to_default_value(schema_type: SchemaType) -> String {
    match schema_type {
        SchemaType::Boolean => "false".to_string(),
        SchemaType::Integer => "0".to_string(),
        SchemaType::Number => "0.0".to_string(),
        SchemaType::String => "".to_string(),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
    }
}
