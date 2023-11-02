use oas3::{spec::SchemaType, Schema, Spec};

#[derive(Clone, Debug)]
pub struct ParsedSchema {
    pub name: Option<String>,
    pub schema_type: SchemaType,
    pub properties: Option<Vec<ParsedSchema>>,
}

pub fn parse_schema(
    named_schemas: Vec<(Option<String>, Schema)>,
    spec: &Spec,
) -> Vec<ParsedSchema> {
    named_schemas
        .iter()
        .map(|(name, schema)| -> ParsedSchema {
            match schema.schema_type.unwrap() {
                SchemaType::Object => parse_object(schema, spec, name),
                _ => ParsedSchema {
                    name: name.clone(),
                    schema_type: schema.schema_type.unwrap(),
                    properties: None,
                },
            }
        })
        .collect()
}

fn parse_object(schema: &Schema, spec: &Spec, name: &Option<String>) -> ParsedSchema {
    let properties: Vec<(Option<String>, Schema)> = schema
        .properties
        .iter()
        .map(
            |(name, property_schema_reference)| -> (Option<String>, Schema) {
                (
                    Some(name.to_string()),
                    property_schema_reference.resolve(&spec).unwrap(),
                )
            },
        )
        .collect();

    let parsed_properties: Vec<ParsedSchema> = parse_schema(properties, &spec)
        .iter()
        .map(|parsed_schema| -> ParsedSchema { parsed_schema.clone() })
        .collect();

    ParsedSchema {
        name: name.clone(),
        properties: Some(parsed_properties),
        schema_type: schema.schema_type.unwrap(),
    }
}

pub fn to_string_schema(schema_type: SchemaType, struct_name: Option<String>) -> String {
    match schema_type {
        SchemaType::Boolean => "bool".to_string(),
        SchemaType::Integer => "i32".to_string(),
        SchemaType::Number => "f32".to_string(),
        SchemaType::String => "String".to_string(),
        SchemaType::Array => panic!("function does not handle schema type \"array\""),
        SchemaType::Object => struct_name.unwrap(),
    }
}
