use super::parse_schema::parse_schema;
use super::ResponseSpec;
use super::SpecInfo;
use oas3::spec::Operation;
use oas3::spec::Response;
use oas3::Spec;
use std::collections::BTreeMap;

pub fn get_response_specs(
    spec_infos_with_operation: Vec<(SpecInfo, Operation)>,
) -> BTreeMap<(String, String), Vec<ResponseSpec>> {
    spec_infos_with_operation
        .iter()
        .map(
            |(spec_info, operation)| -> (String, String, Spec, BTreeMap<String, Response>) {
                let status_code_and_responses = operation.responses(&spec_info.spec);

                (
                    spec_info.name.to_string(),
                    operation.operation_id.clone().unwrap(),
                    spec_info.spec.clone(),
                    status_code_and_responses,
                )
            },
        )
        .map(|(spec_name, operation_id, spec, responses)| {
            let response_specs = responses
                .iter()
                .map(|(status_code, response)| {
                    // TODO: specifically select application/json
                    let (_, media_type) = response.clone().content.pop_first().unwrap();
                    let schema = media_type.schema(&spec).unwrap();

                    let parsed_schema = parse_schema(schema, &spec);

                    let headers: BTreeMap<String, String> = response
                        .clone()
                        .headers
                        .into_iter()
                        .map(|(key, value_ref)| (key, String::new())) // TODO: use real value when oas3 crate fixes ability to resolve headers.
                        .collect();

                    ResponseSpec {
                        status_code: status_code.to_string(),
                        body: parsed_schema,
                        headers,
                    }
                })
                .collect();

            ((spec_name, operation_id), response_specs)
        })
        .collect()
}
