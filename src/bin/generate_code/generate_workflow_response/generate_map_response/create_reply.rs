use codegen::Function;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::spec_parsing::ParsedSchema;

pub fn create_reply(
    function: &mut Function,
    struct_name: String,
    response_values: Vec<(String, ParsedSchema)>,
    status_code: String,
    input_map: Map<String, Value>,
    query_parameters: Vec<(String, SchemaType)>,
) {
    function.line(format!("reply::json(&{} {{", struct_name));

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
