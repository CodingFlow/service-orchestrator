use crate::{
    generate_workflows::input_map::{variable_aliases::Location, InputMap, Variable},
    parse_specs::{
        parse_schema::{to_string_schema, ParsedSchema},
        OperationSpec, RequestSpec,
    },
    traversal::NestedNode,
};
use http::Method;
use oas3::spec::SchemaType;

use super::{
    build_service_call_view_data::generate_response_variables::ResponseAlias,
    create_request_aliases::create_request_aliases,
    create_response_aliases::create_response_aliases, variables::VariableAliases,
};

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
    pub body: Option<NestedNode<ResponseAlias>>,
    pub query_struct_name: String,
}

#[derive(Debug, Clone)]
pub struct WorkflowPathPart {
    pub name: String,
    pub alias: Option<String>,
    pub formatted_type: Option<String>,
}

pub fn build_workflow_request_view_data(
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
                body,
                ..
            },
        ..
    } = operation_spec;

    let workflow_path = create_workflow_path(path, input_map, &operation_id);

    let workflow_query = create_workflow_query(query, input_map, operation_id.to_string());

    let workflow_body = create_workflow_body(body, input_map, variable_aliases, operation_id);

    WorkflowRequestSpec {
        method: method.clone(),
        query: workflow_query,
        path: workflow_path,
        body: workflow_body,
        query_struct_name: variable_aliases.create_alias(),
    }
}

fn create_workflow_body(
    body: Option<NestedNode<ParsedSchema>>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    operation_id: String,
) -> Option<NestedNode<ResponseAlias>> {
    match body {
        Some(body) => {
            let aliases = create_request_aliases(
                body,
                input_map,
                variable_aliases,
                (operation_id, "resonse".to_string(), None, Location::Body),
            );

            Some(aliases)
        }
        None => None,
    }
}

fn create_workflow_query(
    query: std::collections::BTreeMap<String, SchemaType>,
    input_map: &mut InputMap,
    operation_id: String,
) -> Vec<RequestParameter> {
    query
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
        .collect()
}

fn create_workflow_path(
    path: Vec<crate::parse_specs::PathPart>,
    input_map: &mut InputMap,
    operation_id: &String,
) -> Vec<WorkflowPathPart> {
    path.iter()
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
        .collect()
}
