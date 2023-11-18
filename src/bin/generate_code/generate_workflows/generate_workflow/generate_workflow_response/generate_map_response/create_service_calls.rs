mod build_loopkup_map;
mod generate_service_calls;
mod generate_service_response_structs;

use std::{collections::BTreeMap, fs};

use build_loopkup_map::build_service_operation_lookup_map;
use codegen::{Function, Scope};
use generate_service_calls::generate_service_calls;
use generate_service_response_structs::generate_service_response_structs;

use serde_json::Value;
use url::Url;

use crate::{
    generate_workflows::{
        generate_workflow::variables::VariableAliases,
        input_map::{InputMap, InputMapBehavior},
    },
    parse_specs::{get_operation_specs, OperationSpec, SpecType},
};

use self::build_loopkup_map::ServiceCodeGenerationInfo;

pub fn create_service_calls(
    function: &mut Function,
    input_map: &mut InputMap,
    workflow_name: String,
    scope: &mut Scope,
    variable_aliases: &mut VariableAliases,
) {
    let service_urls = get_service_urls();
    let operation_specs = get_operation_specs(SpecType::Service);

    let used_operation_specs =
        filter_to_used_operation_specs(workflow_name.to_string(), operation_specs, &input_map);

    let response_struct_names =
        create_response_struct_names(used_operation_specs.iter(), variable_aliases);

    generate_service_response_structs(
        scope,
        used_operation_specs.clone(),
        response_struct_names.to_vec(),
    );

    let generation_infos = build_service_operation_lookup_map(
        used_operation_specs,
        response_struct_names,
        variable_aliases,
        workflow_name.to_string(),
        service_urls,
        input_map,
    );

    let workflow_response_info = build_workflow_response_lookup_map(
        generation_infos.0.clone(),
        variable_aliases,
        workflow_name,
        input_map,
    );

    generate_stream_enum(scope, generation_infos.clone());

    generate_service_calls(
        scope,
        function,
        generation_infos,
        workflow_response_info,
        variable_aliases,
    );
}

fn get_service_urls() -> BTreeMap<String, Url> {
    let file = match fs::File::open("./src/service_config.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read service configuration file."),
    };

    let config: Value = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse service configuration file."),
    };

    config
        .as_object()
        .unwrap()
        .into_iter()
        .map(|(service_name, value)| {
            (
                service_name,
                value.as_object().unwrap().get("service_url").unwrap(),
            )
        })
        .map(|(service_name, value)| {
            let url = value.as_str().unwrap();
            (service_name.to_string(), Url::parse(url).unwrap())
        })
        .collect()
}

pub struct WorkflowResponseCodeGenerationInfo {
    pub dependency_infos: Vec<WorkflowResponseCodeGenerationDependencyInfo>,
}

pub struct WorkflowResponseCodeGenerationDependencyInfo {
    pub result_destructure_variable_name: String,
    pub dependencies_local_variable_name: String,
    pub service_operation_dependency: ServiceCodeGenerationInfo,
}

fn build_workflow_response_lookup_map(
    generation_infos: BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    variable_aliases: &mut VariableAliases,
    workflow_name: String,
    input_map: &mut InputMap,
) -> WorkflowResponseCodeGenerationInfo {
    let dependencies_ids = input_map.get_workflow_response_dependencies_ids(workflow_name);
    let dependencies_ids_iter = dependencies_ids.iter();

    let result_destructure_variable_names =
        create_variable_names(dependencies_ids_iter.clone(), variable_aliases);

    let dependencies_local_variable_names =
        create_variable_names(dependencies_ids_iter.clone(), variable_aliases);

    let mut service_operation_dependencies = dependencies_ids_iter
        .clone()
        .map(|id| generation_infos.get(id).unwrap());

    let mut dependency_infos = vec![];

    for _ in dependencies_ids_iter {
        dependency_infos.push(WorkflowResponseCodeGenerationDependencyInfo {
            result_destructure_variable_name: result_destructure_variable_names
                .iter()
                .next()
                .unwrap()
                .to_string(),
            dependencies_local_variable_name: dependencies_local_variable_names
                .iter()
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

fn generate_stream_enum(
    scope: &mut Scope,
    (generation_infos_with_id, ordered_generation_infos_with_id): (
        BTreeMap<(std::string::String, std::string::String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
) {
    let struct_names_and_enums: Vec<(String, String)> = ordered_generation_infos_with_id
        .iter()
        .map(|(_, info)| {
            (
                info.enum_name.to_string(),
                info.response_struct_name.to_string(),
            )
        })
        .collect();

    let new_enum = scope.new_enum("Message");

    for (enum_name, response_struct_name) in struct_names_and_enums {
        let variant = new_enum.new_variant(&enum_name);

        variant.tuple(&format!("Result<{}, StatusCode>", response_struct_name));
    }
}

fn create_response_struct_names(
    iter: std::slice::Iter<'_, OperationSpec>,
    variable_aliases: &mut VariableAliases,
) -> Vec<String> {
    iter.clone()
        .map(|_| variable_aliases.create_alias())
        .collect()
}

fn filter_to_used_operation_specs(
    workflow_name: String,
    operation_specs: Vec<OperationSpec>,
    input_map: &InputMap,
) -> Vec<OperationSpec> {
    let service_operation_names = input_map.get_workflow_services_operations_ids(workflow_name);

    operation_specs
        .into_iter()
        .filter(|spec| {
            (&service_operation_names)
                .into_iter()
                .any(|service_operation_name| {
                    *service_operation_name
                        == (spec.spec_name.to_string(), spec.operation_id.to_string())
                })
        })
        .collect()
}
