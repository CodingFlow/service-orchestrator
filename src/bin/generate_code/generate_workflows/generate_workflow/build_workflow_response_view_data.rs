use super::super::input_map::InputMap;
use super::build_service_call_view_data::generate_body_variables::BodyPropertyAlias;
use super::create_workflow_response_aliases::create_workflow_response_aliases;
use super::variables::VariableAliases;
use crate::generate_workflows::input_map::Location;
use crate::parse_specs::OperationSpec;
use crate::traversal::NestedNode;

#[derive(Debug, Clone)]
pub struct WorkflowResponseGenerationInfo {
    pub generation_infos: Vec<ResponseGenerationInfo>,
}

#[derive(Debug, Clone)]
pub struct ResponseGenerationInfo {
    pub body: NestedNode<BodyPropertyAlias>,
    pub headers: Vec<Header>,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub alias: String,
}

pub fn build_workflow_response_view_data(
    operation_spec: &OperationSpec,
    input_map: &mut InputMap,
    variable_aliases: &mut VariableAliases,
) -> WorkflowResponseGenerationInfo {
    let bodies = create_workflow_response_aliases(
        vec![operation_spec.clone()].iter(),
        input_map,
        variable_aliases,
    );

    let headers = create_workflow_headers(operation_spec.clone(), input_map);

    let generation_infos = create_generation_infos(bodies, headers);

    WorkflowResponseGenerationInfo { generation_infos }
}

fn create_workflow_headers(
    workflow_spec: OperationSpec,
    input_map: &mut InputMap,
) -> Vec<Vec<Header>> {
    workflow_spec
        .response_specs
        .iter()
        .map(|response_spec| {
            let namespace = (
                workflow_spec.spec_name.to_string(),
                "response".to_string(),
                None,
                Location::Header,
            );

            response_spec
                .headers
                .iter()
                .map(|(header_name, _)| {
                    let alias = input_map
                        .get_variable_alias(namespace.clone(), vec![header_name.to_string()]);

                    Header {
                        name: header_name.to_string(),
                        alias,
                    }
                })
                .collect::<Vec<Header>>()
        })
        .collect()
}

fn create_generation_infos(
    bodies: Vec<NestedNode<BodyPropertyAlias>>,
    headers: Vec<Vec<Header>>,
) -> Vec<ResponseGenerationInfo> {
    let mut bodies_iter = bodies.iter();
    let mut headers_iter = headers.iter();

    let mut response_generation_infos = vec![];

    for _ in 0..bodies.len() {
        let body = bodies_iter.next().unwrap().clone();
        let headers = headers_iter.next().unwrap().clone();

        let info = ResponseGenerationInfo { body, headers };

        response_generation_infos.push(info);
    }

    response_generation_infos
}
