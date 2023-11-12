use http::Method;
use oas3::{
    spec::{Operation, Parameter, PathItem, SchemaType},
    Schema, Spec,
};

use crate::parse_specs::{OperationSpec, RequestSpec, ResponseSpec};

use super::input_map::{InputMap, InputMapBehavior, Variable};

#[derive(Debug, Clone)]
pub struct RequestParameters {
    pub path_parameters: Vec<RequestParameter>,
    pub query_parameters: Vec<RequestParameter>,
}

#[derive(Debug, Clone)]
pub struct RequestParameter {
    pub name: Variable,
    pub schema_type: SchemaType,
}

#[derive(Debug, Clone)]
pub struct WorkflowOperationSpec {
    pub spec_name: String,
    pub operation_id: String,
    pub request_spec: WorkflowRequestSpec,
    pub response_spec: Vec<ResponseSpec>,
}

#[derive(Debug, Clone)]
pub struct WorkflowRequestSpec {
    pub method: Method,
    pub query: Vec<RequestParameter>,
    pub path: Vec<WorkflowPathPart>,
}

#[derive(Debug, Clone)]
pub struct WorkflowPathPart {
    pub name: WorkflowVariable,
    pub schema_type: Option<SchemaType>,
}

#[derive(Debug, Clone)]
pub enum WorkflowVariable {
    Name(String),
    Variable(Variable),
}

pub fn add_variable_aliases_to_request_parameters(
    operation_spec: OperationSpec,
    input_map: &mut InputMap,
) -> WorkflowOperationSpec {
    let OperationSpec {
        spec_name,
        operation_id,
        request_spec:
            RequestSpec {
                method,
                query,
                path,
                ..
            },
        response_spec,
        ..
    } = operation_spec;

    let workflow_path = path
        .iter()
        .map(|path_part| {
            let name = match path_part.parameter_info {
                Some(_) => WorkflowVariable::Variable(
                    input_map.create_variable_alias(path_part.name.to_string()),
                ),
                None => WorkflowVariable::Name(path_part.name.to_string()),
            };

            let schema_type = match &path_part.parameter_info {
                Some(parameter_info) => {
                    Some(parameter_info.schema.clone().unwrap().schema_type.unwrap())
                }
                None => None,
            };

            WorkflowPathPart { name, schema_type }
        })
        .collect();

    let workflow_query = query
        .iter()
        .map(|(name, schema_type)| RequestParameter {
            name: input_map.create_variable_alias(name.to_string()),
            schema_type: schema_type.clone(),
        })
        .collect();

    WorkflowOperationSpec {
        spec_name: spec_name.to_string(),
        operation_id: operation_id.to_string(),
        request_spec: WorkflowRequestSpec {
            method: method.clone(),
            query: workflow_query,
            path: workflow_path,
        },
        response_spec: response_spec.clone(),
    }
}

fn extract_parameters(
    path_item: &PathItem,
    operation: &Operation,
    spec: &Spec,
    input_map: &mut InputMap,
) -> (Vec<RequestParameter>, Vec<RequestParameter>) {
    let mut all_parameters = path_item.parameters.to_vec();

    all_parameters.extend(operation.parameters.to_vec());

    let all_resolved_parameters: Vec<Parameter> = all_parameters
        .iter()
        .map(|reference| -> Parameter { reference.resolve(&spec).unwrap() })
        .collect();

    let path_parameters = filter_map_parameters(
        input_map,
        all_resolved_parameters.to_vec(),
        |parameter| -> bool { parameter.location == "path" },
    );

    let query_parameters =
        filter_map_parameters(input_map, all_resolved_parameters, |parameter| -> bool {
            parameter.location == "query"
        });

    (path_parameters, query_parameters)
}

fn filter_map_parameters(
    input_map: &mut InputMap,
    all_resolved_parameters: Vec<Parameter>,
    filter_function: fn(&Parameter) -> bool,
) -> Vec<RequestParameter> {
    let path_parameters: Vec<RequestParameter> = all_resolved_parameters
        .to_vec()
        .into_iter()
        .filter(filter_function)
        .map(|parameter| parameter.clone())
        .map(|parameter| -> (String, Schema) { (parameter.name, parameter.schema.unwrap()) })
        .map(|(name, schema)| -> RequestParameter {
            RequestParameter {
                name: input_map.create_variable_alias(name.to_string()),
                schema_type: schema.schema_type.unwrap(),
            }
        })
        .collect();
    path_parameters
}
