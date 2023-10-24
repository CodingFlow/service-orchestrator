use oas3::{spec::SchemaType, Schema, Spec};

#[derive(Clone, Debug)]
pub struct ParsedSchema {
    name: String,
    schema_type: SchemaType,
    properties: Option<Vec<(String, ParsedSchema)>>,
}

pub fn parse_schema(
    named_schemas: Vec<(Option<String>, Schema)>,
    spec: &Spec,
) -> Vec<(Option<String>, ParsedSchema)> {
    named_schemas
        .iter()
        .map(|(name, schema)| -> (Option<String>, ParsedSchema) {
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

            let parsed_properties: Vec<(String, ParsedSchema)> = parse_schema(properties, &spec)
                .iter()
                .map(|(name, parsed_schema)| -> (String, ParsedSchema) {
                    (name.as_deref().unwrap().to_string(), parsed_schema.clone())
                })
                .collect();

            (
                name.clone(),
                ParsedSchema {
                    name: name.as_deref().unwrap().to_string(),
                    properties: Some(parsed_properties),
                    schema_type: schema.schema_type.unwrap(),
                },
            )
        })
        .collect()
}

fn to_string_schema_type_primitive(schema_type: SchemaType) -> &'static str {
    match schema_type {
        SchemaType::Boolean => "bool",
        SchemaType::Integer => "i32",
        SchemaType::Number => "f32",
        SchemaType::String => "String",
        SchemaType::Array => "array",
        SchemaType::Object => panic!("function does not handle schema type object"),
    }
}
