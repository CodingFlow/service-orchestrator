use codegen::{Field, Scope, Struct};
use oas3::spec::SchemaType;

use crate::spec_parsing::to_string_schema_type_primitive;

const QUERY_STRUCT_NAME: &str = "QueryParameters";

pub fn generate_query_struct(
    scope: &mut Scope,
    query_parameters: Vec<(String, SchemaType)>,
) -> &'static str {
    let fields = query_parameters.iter().map(|(name, schema_type)| -> Field {
        let converted_type = to_string_schema_type_primitive(*schema_type);
        Field::new(
            &format!("pub {}", name),
            format!("Option<{}>", converted_type),
        )
    });

    let mut new_struct = Struct::new(QUERY_STRUCT_NAME);

    new_struct
        .vis("pub")
        .derive("Serialize")
        .derive("Deserialize");

    for field in fields {
        new_struct.push_field(field);
    }

    scope.push_struct(new_struct);

    QUERY_STRUCT_NAME
}
