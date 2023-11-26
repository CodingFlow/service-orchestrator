use super::ServiceRequest;
use super::ServiceRequestPath;
use crate::generate_workflows::generate_workflow::create_body_aliases::create_body_aliases;
use crate::generate_workflows::generate_workflow::create_body_aliases::AliasLocation;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::OperationSpec;
use crate::parse_specs::RequestSpec;

pub fn map_requests_with_variables(
    iter: std::slice::Iter<'_, OperationSpec>,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
) -> Vec<ServiceRequest> {
    iter.map(|operation_spec| {
        let request_spec = operation_spec.request_spec.clone();
        let query = create_query(
            request_spec.clone(),
            input_map,
            &workflow_name,
            operation_spec,
        );
        let path = create_path(
            request_spec.clone(),
            input_map,
            workflow_name.to_string(),
            operation_spec,
        );

        let body = match request_spec.body {
            Some(body) => {
                let namespace = (
                    workflow_name.to_string(),
                    operation_spec.spec_name.to_string(),
                    Some(operation_spec.operation_id.to_string()),
                    Location::Body,
                );

                let body_aliases = create_body_aliases(
                    body,
                    input_map,
                    variable_aliases,
                    namespace,
                    AliasLocation::Destination,
                );

                Some(body_aliases)
            }
            None => None,
        };

        ServiceRequest {
            method: request_spec.method,
            query,
            path,
            body,
        }
    })
    .collect()
}

fn create_path(
    request_spec: RequestSpec,
    input_map: &InputMap,
    workflow_name: String,
    operation_spec: &OperationSpec,
) -> Vec<ServiceRequestPath> {
    request_spec
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
        .collect()
}

fn create_query(
    request_spec: RequestSpec,
    input_map: &InputMap,
    workflow_name: &String,
    operation_spec: &OperationSpec,
) -> std::collections::BTreeMap<String, String> {
    request_spec
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
        .collect()
}
