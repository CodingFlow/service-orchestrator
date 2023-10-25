use std::collections::BTreeMap;
use std::fs;

use codegen::{Field, Scope, Struct};
use oas3::Spec;

use oas3::spec::{ObjectOrReference, Response};

use crate::spec_parsing::{parse_schema, to_string_schema_type_primitive, ParsedSchema};

pub fn generate_workflow_response(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
) {
    let mut scope = Scope::new();

    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");

    let response_values: BTreeMap<String, Response> =
        extract_response_values_from_spec(responses, spec);

    let status_code_struct_name_pairs =
        generate_response_structure(response_values, &mut scope, &spec);

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn generate_response_structure(
    response_values: BTreeMap<String, Response>,
    scope: &mut Scope,
    spec: &Spec,
) -> Vec<(String, String)> {
    let responses: Vec<(String, String, Struct)> = response_values
        .iter()
        .map(|(status_code, response_value)| -> (&String, ParsedSchema) {
            let btree_map = &mut response_value.content.clone();
            let first_entry = btree_map.first_entry();
            let occupied_entry = &first_entry.unwrap();
            let json_response = occupied_entry.get();
            let parsed_schemas = parse_schema(
                vec![(
                    Some(status_code.to_string()),
                    json_response.schema.clone().unwrap().resolve(spec).unwrap(),
                )],
                spec,
            );

            (status_code, parsed_schemas.first().unwrap().clone())
        })
        .enumerate()
        .map(
            |(index, (status_code, parsed_schema))| -> (String, String, Struct) {
                let struct_name = &format!("WorkflowResponse{}", index);
                let mut new_struct = Struct::new(struct_name);

                new_struct.derive("Serialize").derive("Deserialize");

                let fields: Vec<Field> = parsed_schema
                    .properties
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
            },
        )
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

fn extract_response_values_from_spec(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
) -> BTreeMap<String, Response> {
    let a = responses.clone();

    a.iter()
        .map(|(status_code, wrapped_response)| -> (String, Response) {
            (
                status_code.to_string(),
                wrapped_response.resolve(spec).unwrap(),
            )
        })
        .collect()
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_response_definition_test.rs", code);
}
