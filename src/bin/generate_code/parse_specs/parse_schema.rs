use oas3::{spec::SchemaType, Schema, Spec};

use crate::traversal::{convert_to_nested_node, NestedNode};

#[derive(Clone, Debug)]
pub struct ParsedSchema {
    pub name: Option<String>,
    pub schema_type: SchemaType,
}

pub fn parse_schema(schema: Schema, mut spec: &Spec) -> NestedNode<ParsedSchema> {
    convert_to_nested_node(
        (None, schema),
        |(name, schema), _| ParsedSchema {
            name,
            schema_type: schema.schema_type.unwrap(),
        },
        |(_, schema), spec| match schema.schema_type {
            Some(_) => Some(
                schema
                    .properties
                    .into_iter()
                    .map(|(name, schema_ref)| (Some(name), schema_ref.resolve(spec).unwrap()))
                    .collect::<Vec<(Option<String>, Schema)>>(),
            ),
            None => None,
        },
        &mut spec,
    )
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
