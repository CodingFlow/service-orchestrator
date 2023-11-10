mod extract_request_parameters_from_spec;
mod generate_workflow;
mod get_endpoint_infos_from_specs;
mod input_map;

use crate::{
    generate_create_filter::generate_create_filter, generate_re_exports::ReExports,
    parse_specs::SpecInfo,
};

use extract_request_parameters_from_spec::extract_request_parameters_from_spec;
use generate_workflow::generate_workflow;
use get_endpoint_infos_from_specs::get_endpoint_infos_from_specs;

use self::input_map::create_input_map;

pub fn generate_workflows(workflow_spec_infos: Vec<SpecInfo>, re_exports: &mut ReExports) {
    let mut workflow_definition_names = vec![];
    let mut input_map = create_input_map();

    let endpoint_infos = get_endpoint_infos_from_specs(workflow_spec_infos);

    for (spec_info, path_string, path_item, method, operation) in endpoint_infos {
        let request_parameters = extract_request_parameters_from_spec(
            &path_item,
            &operation,
            &spec_info.spec,
            &mut input_map,
        );

        let names = generate_workflow(
            request_parameters,
            &operation,
            &spec_info,
            method,
            &path_string,
            &input_map,
            re_exports,
        );

        workflow_definition_names.push(names);
    }

    generate_create_filter(workflow_definition_names, re_exports);
}
