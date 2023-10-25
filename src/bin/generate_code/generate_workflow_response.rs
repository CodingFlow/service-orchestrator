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
        .map(
            |(status_code, response_value)| -> (&String, Vec<ParsedSchema>) {
                let btree_map = &mut response_value.content.clone();
                let first_entry = btree_map.first_entry();
                let occupied_entry = &first_entry.unwrap();
                let json_response = occupied_entry.get();
                let properties = parse_schema(
                    vec![(
                        None,
                        json_response.schema.clone().unwrap().resolve(spec).unwrap(),
                    )],
                    spec,
                );

                (status_code, properties)
            },
        )
        .enumerate()
        .map(
            |(index, (status_code, schemas))| -> (String, String, Struct) {
                let struct_name = &format!("WorkflowResponse{}", index);
                let mut new_struct = Struct::new(struct_name);

                let fields: Vec<Field> = schemas
                    .iter()
                    .map(|schema| -> Field {
                        Field::new(
                            schema.name.clone().unwrap().as_str(),
                            to_string_schema_type_primitive(schema.schema_type),
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
    let _ = fs::write("./src/workflow_response_definition.rs", code);
}
