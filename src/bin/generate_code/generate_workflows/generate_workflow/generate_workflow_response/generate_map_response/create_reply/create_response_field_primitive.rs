use codegen::Function;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::spec_parsing::ParsedSchema;

pub fn create_response_field_primitive(
    function: &mut Function,
    response_property: ParsedSchema,
    input_map: &Map<String, Value>,
    query_parameters: &Vec<(String, SchemaType)>,
) {
    let query_parameters = query_parameters.to_vec();
    let property_name = response_property.name.unwrap();
    let mapped_value_name = input_map.get(&property_name).unwrap().as_str().unwrap();

    function.line(format!(
        "{}:{},",
        property_name,
        format_response_field_value(
            response_property.schema_type,
            query_parameters,
            mapped_value_name
        ),
    ));
}

fn format_response_field_value(
    response_property_schema_type: SchemaType,
    query_parameters: Vec<(String, SchemaType)>,
    mapped_value_name: &str,
) -> String {
    let response_property_schema_type = response_property_schema_type;
    let mapped_value_name = mapped_value_name;
    match is_query_parameter(query_parameters, mapped_value_name) {
        true => match response_property_schema_type {
            SchemaType::String => format!(
                "{}.get_or_insert({}).to_string()",
                mapped_value_name,
                convert_type_to_default_value(response_property_schema_type)
            ),
            _ => format!(
                "*{}.get_or_insert({})",
                mapped_value_name,
                convert_type_to_default_value(response_property_schema_type)
            ),
        },
        false => mapped_value_name.to_string(),
    }
}

fn is_query_parameter(
    query_parameters: Vec<(String, SchemaType)>,
    mapped_value_name: &str,
) -> bool {
    query_parameters
        .iter()
        .any(|(name, _)| -> bool { name.to_string() == mapped_value_name })
}

fn convert_type_to_default_value(schema_type: SchemaType) -> String {
    match schema_type {
        SchemaType::Boolean => "false".to_string(),
        SchemaType::Integer => "0".to_string(),
        SchemaType::Number => "0.0".to_string(),
        SchemaType::String => "String::new()".to_string(),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
    }
}
