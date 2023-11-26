mod create_dependencies;
mod create_service_code_generation_infos;
mod create_variable_names;
mod map_requests_with_variables;
mod order_by_dependencies;

use super::generate_response_variables::ResponseAlias;
use crate::generate_workflows::generate_workflow::create_response_aliases::create_response_aliases;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::parse_specs::OperationSpec;
use crate::traversal::NestedNode;
use create_dependencies::create_dependencies;
use create_service_code_generation_infos::create_service_code_generation_infos;
use create_variable_names::create_variable_names;
use http::Method;
use map_requests_with_variables::map_requests_with_variables;
use order_by_dependencies::order_by_dependencies;
use std::collections::BTreeMap;
use url::Url;

#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub method: Method,
    pub query: BTreeMap<String, String>,
    pub path: Vec<ServiceRequestPath>,
    pub body: Option<NestedNode<ResponseAlias>>,
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
    pub dependencies_service_operation_names: Vec<(String, String)>,
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

    let requests =
        map_requests_with_variables(iter.clone(), input_map, variable_aliases, workflow_name);

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
