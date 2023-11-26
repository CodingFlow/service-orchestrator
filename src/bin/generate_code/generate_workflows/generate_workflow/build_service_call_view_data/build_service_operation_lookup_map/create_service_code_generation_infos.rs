use super::super::generate_response_variables::ResponseAlias;
use super::ServiceCodeGenerationInfo;
use super::ServiceRequest;
use crate::parse_specs::OperationSpec;
use crate::traversal::NestedNode;
use std::collections::BTreeMap;
use url::Url;

pub fn create_service_code_generation_infos(
    iter: std::slice::Iter<'_, OperationSpec>,
    future_variable_names: Vec<String>,
    enum_names: Vec<String>,
    stream_variable_names: Vec<String>,
    response_aliases: Vec<NestedNode<ResponseAlias>>,
    dependencies: BTreeMap<(String, String), Vec<(String, String)>>,
    requests: Vec<ServiceRequest>,
    service_urls: BTreeMap<String, Url>,
) -> BTreeMap<(String, String), ServiceCodeGenerationInfo> {
    let mut future_variable_names_iter = future_variable_names.iter();
    let mut enum_names_iter = enum_names.iter();
    let mut stream_variable_names_iter = stream_variable_names.iter();
    let mut response_aliases_iter = response_aliases.iter();
    let mut dependencies_iter = dependencies.iter().map(|(_, dependencies)| dependencies);
    let mut requests_iter = requests.iter();

    iter.map(|operation_spec| {
        let future_variable_name = future_variable_names_iter.next().unwrap().to_string();
        let enum_name = enum_names_iter.next().unwrap().to_string();
        let stream_variable_name = stream_variable_names_iter.next().unwrap().to_string();
        let response_aliases = response_aliases_iter.next().unwrap().clone();
        let dependencies_service_operation_names = dependencies_iter.next().unwrap().to_vec();
        let request = requests_iter.next().unwrap().clone();
        let service_url = service_urls.get(&operation_spec.spec_name).unwrap().clone();

        (
            (
                operation_spec.spec_name.to_string(),
                operation_spec.operation_id.to_string(),
            ),
            ServiceCodeGenerationInfo {
                future_variable_name,
                enum_name,
                stream_variable_name,
                response_aliases,
                dependencies_service_operation_names,
                request,
                service_url,
            },
        )
    })
    .collect()
}
