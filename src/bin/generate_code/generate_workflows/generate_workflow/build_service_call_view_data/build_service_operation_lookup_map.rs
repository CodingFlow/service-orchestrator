use super::generate_response_variables::ResponseAlias;
use crate::generate_workflows::generate_workflow::create_response_aliases::create_response_aliases;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::variable_aliases::Location;
use crate::generate_workflows::input_map::InputMap;
use crate::parse_specs::OperationSpec;
use crate::traversal::NestedNode;
use http::Method;
use std::collections::BTreeMap;
use url::Url;

#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub method: Method,
    pub query: BTreeMap<String, String>,
    pub path: Vec<ServiceRequestPath>,
}

#[derive(Debug, Clone)]
pub struct ServiceRequestPath {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ServiceCodeGenerationInfo {
    pub future_variable_name: String,
    pub enum_name: String,
    pub stream_variable_name: String,
    pub response_aliases: NestedNode<ResponseAlias>,
    pub dependencies_service_names: Vec<(String, String)>,
    pub request: ServiceRequest,
    pub service_url: Url,
}

pub fn build_service_operation_lookup_map(
    operation_specs: Vec<OperationSpec>,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
    service_urls: BTreeMap<String, Url>,
    input_map: &mut InputMap,
) -> (
    BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    Vec<((String, String), ServiceCodeGenerationInfo)>,
) {
    let iter = operation_specs.iter();

    let future_variable_names = create_variable_names(iter.clone(), variable_aliases);

    let enum_names = create_variable_names(iter.clone(), variable_aliases);

    let stream_variable_names = create_variable_names(iter.clone(), variable_aliases);

    let response_aliases = create_response_aliases(
        iter.clone(),
        input_map,
        variable_aliases,
        workflow_name.to_string(),
    );

    let dependencies = create_dependencies(input_map, workflow_name.to_string());

    let requests = map_requests_with_variables(iter.clone(), input_map, workflow_name);

    let code_generation_infos = create_service_code_generation_infos(
        iter,
        future_variable_names,
        enum_names,
        stream_variable_names,
        response_aliases,
        dependencies,
        requests,
        service_urls,
    );

    let ordered = order_by_dependencies(code_generation_infos.clone());

    (code_generation_infos, ordered)
}

fn create_variable_names(
    iter: std::slice::Iter<'_, OperationSpec>,
    variable_aliases: &mut VariableAliases,
) -> Vec<String> {
    iter.clone()
        .map(|_| variable_aliases.create_alias())
        .collect()
}

/// Returns [ServiceCodeGenerationInfo]s ordered from most dependent to independent
/// e.g. a -> b -> c then \[a, b, c\]
fn order_by_dependencies(
    code_generation_infos: BTreeMap<(String, String), ServiceCodeGenerationInfo>,
) -> Vec<((String, String), ServiceCodeGenerationInfo)> {
    let mut vector: Vec<((String, String), ServiceCodeGenerationInfo)> =
        code_generation_infos.into_iter().collect();

    let mut complete = false;

    while !complete {
        let vec = &vector.to_vec();
        let iter = vec.iter();
        let cloned_vector: Vec<(usize, &((String, String), ServiceCodeGenerationInfo))> =
            iter.clone().enumerate().collect();

        for (index, (_, info)) in cloned_vector {
            let depending_services = &info.dependencies_service_names;
            let misplaced_depending_service_index =
                iter.clone().take(index).position(|(service_operation, _)| {
                    depending_services
                        .iter()
                        .any(|service| service == service_operation)
                });

            if let Some(misplaced_index) = misplaced_depending_service_index {
                let misplaced_service = vector.remove(misplaced_index);
                vector.insert(index, misplaced_service);
                break;
            }
        }

        complete = true;
    }

    vector
}

fn create_service_code_generation_infos(
    iter: std::slice::Iter<'_, OperationSpec>,
    future_variable_names: Vec<String>,
    enum_names: Vec<String>,
    stream_variable_names: Vec<String>,
    response_aliases: Vec<NestedNode<ResponseAlias>>,
    dependencies: BTreeMap<String, Vec<(String, String)>>,
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
        let dependencies_service_names = dependencies_iter.next().unwrap().to_vec();
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
                dependencies_service_names,
                request,
                service_url,
            },
        )
    })
    .collect()
}

fn map_requests_with_variables(
    iter: std::slice::Iter<'_, OperationSpec>,
    input_map: &InputMap,
    workflow_name: String,
) -> Vec<ServiceRequest> {
    iter.map(|operation_spec| {
        let request_spec = operation_spec.request_spec.clone();
        let query = request_spec
            .query
            .into_iter()
            .map(|(name, _)| {
                let alias = input_map.get_variable_alias(
                    (
                        workflow_name.to_string(),
                        operation_spec.spec_name.to_string(),
                        Some(operation_spec.operation_id.to_string()),
                        Location::Query,
                    ),
                    vec![name.to_string()],
                );
                (name, alias)
            })
            .collect();
        let path = request_spec
            .path
            .iter()
            .map(|path_part| {
                let mut alias = None;

                if let Some(_) = &path_part.parameter_info {
                    alias = Some(input_map.get_variable_alias(
                        (
                            workflow_name.to_string(),
                            operation_spec.spec_name.to_string(),
                            Some(operation_spec.operation_id.to_string()),
                            Location::Path,
                        ),
                        vec![path_part.name.to_string()],
                    ));
                }

                ServiceRequestPath {
                    name: path_part.name.to_string(),
                    alias,
                }
            })
            .collect();

        ServiceRequest {
            method: request_spec.method,
            query,
            path,
        }
    })
    .collect()
}

fn create_dependencies(
    input_map: &InputMap,
    workflow_name: String,
) -> BTreeMap<String, Vec<(String, String)>> {
    input_map.get_service_dependencies(workflow_name.to_string())
}
