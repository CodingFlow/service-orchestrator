use std::collections::BTreeMap;

use oas3::{
    spec::{ObjectOrReference, Response},
    Spec,
};

use crate::spec_parsing::{parse_schema, ParsedSchema};

pub fn parse_responses(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
) -> Vec<(String, ParsedSchema)> {
    let extracted_responses = extract_response_values_from_spec(responses, spec);
    let response_values: Vec<(String, ParsedSchema)> = extracted_responses
        .iter()
        .map(|(status_code, response_value)| -> (String, ParsedSchema) {
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

            (
                status_code.to_string(),
                parsed_schemas.first().unwrap().clone(),
            )
        })
        .collect();

    response_values
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
