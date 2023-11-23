use super::OperationSpec;
use super::RequestSpec;
use super::ResponseSpec;
use std::collections::BTreeMap;

pub fn create_operation_specs(
    request_specs_with_ids: BTreeMap<(String, String), RequestSpec>,
    response_specs_with_ids: BTreeMap<(String, String), Vec<ResponseSpec>>,
) -> Vec<OperationSpec> {
    request_specs_with_ids
        .iter()
        .map(|(key, request_spec)| {
            let matching_response_spec = response_specs_with_ids.get(key).unwrap();
            let (spec_name, operation_id) = key;

            OperationSpec {
                spec_name: spec_name.to_string(),
                operation_id: operation_id.to_string(),
                request_spec: request_spec.clone(),
                response_specs: matching_response_spec.clone(),
            }
        })
        .collect()
}
