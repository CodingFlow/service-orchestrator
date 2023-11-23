mod create_operation_specs;
mod get_directory_path;
mod get_request_spec;
mod get_response_specs;
pub mod parse_schema;
mod parse_specs;

use self::parse_schema::ParsedSchema;
use crate::traversal::NestedNode;
use create_operation_specs::create_operation_specs;
use get_directory_path::get_directory_path;
use get_request_spec::get_request_specs;
use get_response_specs::get_response_specs;
use http::Method;
use oas3::{
    spec::{Header, Parameter, SchemaType},
    Spec,
};
use parse_specs::parse_specs;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct SpecInfo {
    pub name: String,
    pub spec: Spec,
}

pub enum SpecType {
    Workflow,
    Service,
}

#[derive(Debug, Clone)]
pub struct OperationSpec {
    pub spec_name: String,
    pub operation_id: String,
    pub request_spec: RequestSpec,
    pub response_specs: Vec<ResponseSpec>,
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub method: Method,
    pub query: BTreeMap<String, SchemaType>,
    pub path: Vec<PathPart>,
    pub body: Option<NestedNode<ParsedSchema>>,
}

#[derive(Debug, Clone)]
pub struct PathPart {
    pub name: String,
    pub parameter_info: Option<Parameter>,
}

#[derive(Debug, Clone)]
pub struct ResponseSpec {
    pub status_code: String,
    pub body: NestedNode<ParsedSchema>,
    pub headers: BTreeMap<String, String>,
}

pub fn get_operation_specs(spec_type: SpecType) -> Vec<OperationSpec> {
    let directory_path = get_directory_path(spec_type);
    let spec_infos = parse_specs(&directory_path);
    let (request_specs_with_ids, spec_infos_with_operation) = get_request_specs(spec_infos);
    let response_specs_with_ids = get_response_specs(spec_infos_with_operation);

    create_operation_specs(request_specs_with_ids, response_specs_with_ids)
}
