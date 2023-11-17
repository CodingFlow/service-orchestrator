use codegen::{Field, Scope, Struct};

use crate::{
    generate_workflows::generate_workflow::add_variable_aliases_to_request_parameters::RequestParameter,
    parse_specs::parse_schema::to_string_schema,
};

const QUERY_STRUCT_NAME: &str = "QueryParameters";

pub fn generate_query_struct(
    scope: &mut Scope,
    query_parameters: Vec<RequestParameter>,
) -> &'static str {
    let fields = query_parameters.iter().map(|parameter| -> Field {
        let converted_type = to_string_schema(
            parameter.schema_type,
            Some(parameter.name.original_name.to_string()),
        );
        Field::new(
            &format!("pub {}", parameter.name.original_name),
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
