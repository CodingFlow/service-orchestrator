use codegen::{Field, Scope, Struct};

use crate::spec_parsing::{to_string_schema_type_primitive, ParsedSchema};

pub fn generate_response_structure(
    response_values: Vec<(String, ParsedSchema)>,
    scope: &mut Scope,
) -> Vec<(String, String)> {
    let responses: Vec<(String, String, Struct)> = response_values
        .iter()
        .map(|(status_code, parsed_schema)| -> (String, String, Struct) {
            let struct_name = &format!("WorkflowResponse_{}", status_code);
            let mut new_struct = Struct::new(struct_name);

            new_struct.derive("Serialize").derive("Deserialize");

            let fields: Vec<Field> = parsed_schema
                .properties
                .clone()
                .unwrap()
                .iter()
                .map(|property_schema| -> Field {
                    Field::new(
                        &property_schema.name.clone().unwrap(),
                        to_string_schema_type_primitive(property_schema.schema_type), // assumes no nested objects in response
                    )
                })
                .collect();

            for field in fields {
                new_struct.push_field(field);
            }

            (status_code.to_string(), struct_name.to_string(), new_struct)
        })
        .collect();

    let final_result = responses
        .iter()
        .map(|(status_code, struct_name, _)| -> (String, String) {
            (status_code.to_string(), struct_name.to_string())
        })
        .collect();

    for (_, _, structure) in responses {
        scope.push_struct(structure);
    }

    final_result
}
