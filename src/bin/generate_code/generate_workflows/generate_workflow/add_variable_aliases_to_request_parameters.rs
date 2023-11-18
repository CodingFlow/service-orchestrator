use crate::{
    generate_workflows::input_map::{InputMap, InputMapBehavior, Variable},
    parse_specs::{parse_schema::to_string_schema, OperationSpec, RequestSpec, ResponseSpec},
};
use http::Method;
use oas3::spec::SchemaType;

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
    pub name: String,
    pub alias: Option<String>,
    pub formatted_type: Option<String>,
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
        response_specs: response_spec,
        ..
    } = operation_spec;

    let workflow_path = path
        .iter()
        .map(|path_part| {
            let alias = match path_part.parameter_info {
                Some(_) => Some(
                    input_map
                        .create_variable_alias(
                            (operation_id.to_string(), "response".to_string(), None),
                            vec![path_part.name.to_string()],
                        )
                        .alias,
                ),
                None => None,
            };

            let formatted_type = match &path_part.parameter_info {
                Some(parameter_info) => Some(to_string_schema(
                    parameter_info.schema.clone().unwrap().schema_type.unwrap(),
                    None,
                )),
                None => None,
            };

            WorkflowPathPart {
                name: path_part.name.to_string(),
                alias,
                formatted_type,
            }
        })
        .collect();

    let workflow_query = query
        .iter()
        .map(|(name, schema_type)| RequestParameter {
            name: input_map.create_variable_alias(
                (operation_id.to_string(), "response".to_string(), None),
                vec![name.to_string()],
            ),
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
