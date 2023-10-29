use codegen::{Field, Scope, Struct};
use oas3::spec::SchemaType;

use crate::spec_parsing::to_string_schema_type_primitive;

pub fn generate_query_struct(
    scope: &mut Scope,
    query_parameters: Vec<(String, SchemaType)>,
) -> &'static str {
    let fields = query_parameters.iter().map(|(name, schema_type)| -> Field {
        let converted_type = to_string_schema_type_primitive(*schema_type);
        Field::new(name, format!("Option<{}>", converted_type))
    });

    let mut new_struct = Struct::new("QueryParameters");

    new_struct.vis("pub");

    new_struct.derive("Serialize").derive("Deserialize");

    for field in fields {
        new_struct.push_field(field);
    }

    scope.push_struct(new_struct);

    "QueryParameters"
}
