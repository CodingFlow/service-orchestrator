use crate::{
    generate_workflows::input_map::{variable_aliases::Location, InputMap, Variable},
    parse_specs::{parse_schema::to_string_schema, OperationSpec, RequestSpec},
};
use http::Method;
use oas3::spec::SchemaType;

use super::variables::VariableAliases;

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
pub struct WorkflowRequestSpec {
    pub method: Method,
    pub query: Vec<RequestParameter>,
    pub path: Vec<WorkflowPathPart>,
    pub query_struct_name: String,
}

#[derive(Debug, Clone)]
pub struct WorkflowPathPart {
    pub name: String,
    pub alias: Option<String>,
    pub formatted_type: Option<String>,
}

pub fn build_request_view_data(
    operation_spec: OperationSpec,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
) -> WorkflowRequestSpec {
    let OperationSpec {
        operation_id,
        request_spec:
            RequestSpec {
                method,
                query,
                path,
                ..
            },
        ..
    } = operation_spec;

    let workflow_path = path
        .iter()
        .map(|path_part| {
            let alias = match path_part.parameter_info {
                Some(_) => Some(
                    input_map
                        .create_variable_alias(
                            (
                                operation_id.to_string(),
                                "response".to_string(),
                                None,
                                Location::Path,
                            ),
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
                (
                    operation_id.to_string(),
                    "response".to_string(),
                    None,
                    Location::Query,
                ),
                vec![name.to_string()],
            ),
            schema_type: schema_type.clone(),
        })
        .collect();

    WorkflowRequestSpec {
        method: method.clone(),
        query: workflow_query,
        path: workflow_path,
        query_struct_name: variable_aliases.create_alias(),
    }
}
