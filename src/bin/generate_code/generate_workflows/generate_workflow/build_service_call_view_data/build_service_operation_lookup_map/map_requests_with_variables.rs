use super::ServiceRequest;
use super::ServiceRequestPath;
use crate::generate_workflows::input_map::InputMap;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::OperationSpec;

pub fn map_requests_with_variables(
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
