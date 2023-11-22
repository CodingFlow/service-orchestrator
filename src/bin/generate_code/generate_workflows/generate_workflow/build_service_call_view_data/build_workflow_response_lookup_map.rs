use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::generate_workflows::input_map::InputMap;
use std::collections::BTreeMap;

use super::build_service_operation_lookup_map::ServiceCodeGenerationInfo;

pub struct WorkflowResponseCodeGenerationInfo {
    pub dependency_infos: Vec<WorkflowResponseCodeGenerationDependencyInfo>,
}

pub struct WorkflowResponseCodeGenerationDependencyInfo {
    pub result_destructure_variable_name: String,
    pub service_operation_dependency: ServiceCodeGenerationInfo,
}

pub fn build_workflow_response_lookup_map(
    generation_infos: BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
    input_map: &mut InputMap,
) -> WorkflowResponseCodeGenerationInfo {
    let dependencies_ids = input_map.get_workflow_response_dependencies_ids(workflow_name);
    let dependencies_ids_iter = dependencies_ids.iter();

    let result_destructure_variable_names =
        create_variable_names(dependencies_ids_iter.clone(), variable_aliases);
    let result_destructure_variable_name_iter = &mut result_destructure_variable_names.iter();

    let mut service_operation_dependencies = dependencies_ids_iter
        .clone()
        .map(|id| generation_infos.get(id).unwrap());

    let mut dependency_infos = vec![];

    for _ in dependencies_ids_iter {
        dependency_infos.push(WorkflowResponseCodeGenerationDependencyInfo {
            result_destructure_variable_name: result_destructure_variable_name_iter
                .next()
                .unwrap()
                .to_string(),
            service_operation_dependency: service_operation_dependencies.next().unwrap().clone(),
        })
    }

    WorkflowResponseCodeGenerationInfo { dependency_infos }
}

fn create_variable_names(
    iter: std::slice::Iter<'_, (String, String)>,
    variable_aliases: &mut VariableAliases,
) -> Vec<String> {
    iter.clone()
        .map(|_| variable_aliases.create_alias())
        .collect()
}
