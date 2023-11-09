use codegen::Function;
use http::Method;
use oas3::spec::{Operation, PathItem};
use serde_json::{Map, Value};

use crate::{
    generate_workflows::{
        get_endpoint_infos_from_specs::get_endpoint_infos_from_specs,
        input_map::{InputMap, InputMapBehavior},
    },
    parse_specs::{parse_specs, SpecInfo},
};

pub fn create_service_calls(function: &mut Function, input_map: InputMap, workflow_name: String) {
    let service_spec_infos = parse_specs("./src/service_specs/");
    let service_mappings = input_map.get_all_services_for_workflow(workflow_name);
    let mapped_endpoint_infos = filter_service_spec_infos(service_spec_infos, service_mappings);
}

fn filter_service_spec_infos(
    service_spec_infos: Vec<SpecInfo>,
    service_mappings: Map<String, Value>,
) -> Vec<(
    &'static SpecInfo,
    String,
    PathItem,
    Method,
    &'static Operation,
)> {
    let endpoint_infos = get_endpoint_infos_from_specs(service_spec_infos);

    endpoint_infos
        .into_iter()
        .filter(
            |(spec_info, path_string, path_item, method, operation)| -> bool {
                service_mappings.contains_key(&operation.operation_id.unwrap())
            },
        )
        .collect()
}
