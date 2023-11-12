use std::collections::BTreeMap;

use codegen::Function;
use http::Method;
use oas3::spec::{Operation, PathItem};
use serde_json::{Map, Value};

use crate::{
    generate_workflows::{
        get_endpoint_infos_from_specs::get_endpoint_infos_from_specs,
        input_map::{InputMap, InputMapBehavior},
    },
    parse_specs::SpecInfo,
};

pub fn create_service_calls(
    function: &mut Function,
    mut input_map: InputMap,
    workflow_name: String,
) {
    let service_mappings = input_map.get_workflow_services(workflow_name);

    let futures_alias_lookup = BTreeMap::<String, String>::new();
    // Need to create variable aliases for service responses
    // before referencing them in service requests

    // process_response_data(filtered_endpoint_infos, futures_alias_lookup);
    // process_request_data();

    // determine_dependencies();

    // create_response_struct();

    // create_request_response_futures(); // depends on dependencies

    // // streams depends only on futures. Enums depend on response structs and dependencies.
    // // Create enums as part of output: per future, create stream & enum.depends on response structs and dependencies.
    // create_streams_and_enums();
    // create_service_results_for_workflow_response(); // depends on enums, response structs, dependencies

    // generate_all_services() // actually "print" as generated code: keep this dumb.
}

// fn process_response_data(
//     endpoint_infos: Vec<(SpecInfo, String, PathItem, Method, Operation)>,
//     futures_alias_lookup: BTreeMap<String, String>,
// ) -> _ {
// }

fn filter_service_spec_infos(
    service_spec_infos: Vec<SpecInfo>,
    service_mappings: Map<String, Value>,
) -> Vec<(SpecInfo, String, PathItem, Method, Operation)> {
    let endpoint_infos = get_endpoint_infos_from_specs(service_spec_infos);

    endpoint_infos
        .into_iter()
        .filter(
            |(spec_info, path_string, path_item, method, operation)| -> bool {
                service_mappings.contains_key(&operation.operation_id.clone().unwrap())
            },
        )
        .collect()
}
