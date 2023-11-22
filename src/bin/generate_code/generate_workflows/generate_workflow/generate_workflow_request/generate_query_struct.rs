use codegen::{Field, Scope, Struct};

use crate::{
    generate_workflows::generate_workflow::build_workflow_request_view_data::RequestParameter,
    parse_specs::parse_schema::to_string_schema,
};

pub fn generate_query_struct(
    scope: &mut Scope,
    query_parameters: Vec<RequestParameter>,
    query_struct_name: &str,
) {
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

    let mut new_struct = Struct::new(query_struct_name);

    new_struct
        .vis("pub")
        .derive("Serialize")
        .derive("Deserialize");

    for field in fields {
        new_struct.push_field(field);
    }

    scope.push_struct(new_struct);
}
