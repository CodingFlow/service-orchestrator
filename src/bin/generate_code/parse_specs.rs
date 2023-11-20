mod get_request_spec;
pub mod parse_schema;

use std::{collections::BTreeMap, fs::read_dir, path::Path};

use get_request_spec::get_request_specs;
use http::Method;
use oas3::{
    spec::{Operation, Parameter, Response, SchemaType},
    Spec,
};

use parse_schema::parse_schema;

use crate::traversal::NestedNode;

use self::parse_schema::ParsedSchema;

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
}

pub fn get_operation_specs(spec_type: SpecType) -> Vec<OperationSpec> {
    let directory_path = get_directory_path(spec_type);
    let spec_infos = parse_specs(&directory_path);
    let (request_specs_with_ids, spec_infos_with_operation) = get_request_specs(spec_infos);
    let response_specs_with_ids = get_response_specs(spec_infos_with_operation);

    create_operation_specs(request_specs_with_ids, response_specs_with_ids)
}

fn get_response_specs(
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

                    ResponseSpec {
                        status_code: status_code.to_string(),
                        body: parsed_schema,
                    }
                })
                .collect();

            ((spec_name, operation_id), response_specs)
        })
        .collect()
}

fn get_directory_path(spec_type: SpecType) -> String {
    match spec_type {
        SpecType::Workflow => "./src/workflow_specs/".to_string(),
        SpecType::Service => "./src/service_specs/".to_string(),
    }
}

fn parse_specs(specs_directory_path: &str) -> Vec<SpecInfo> {
    let files = match read_dir(Path::new(specs_directory_path)) {
        Ok(files) => files,
        Err(_) => panic!("Unable to read open api spec files!"),
    };

    files
        .map(|dir_entry| -> SpecInfo {
            let dir_entry = &dir_entry.unwrap();

            SpecInfo {
                name: dir_entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                spec: parse_config(dir_entry.path().as_path()),
            }
        })
        .collect()
}

fn parse_config(path: &Path) -> oas3::Spec {
    match oas3::from_path(path) {
        Ok(spec) => spec,
        Err(_) => panic!("unable to read open API spec file"),
    }
}

fn create_operation_specs(
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
