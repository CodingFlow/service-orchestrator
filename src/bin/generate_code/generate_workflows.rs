mod generate_workflow;
mod input_map;

use crate::{
    extract_request_values_from_spec::extract_request_parameters_from_spec,
    generate_create_filter::generate_create_filter, generate_re_exports::ReExports,
    parse_workflow_specs::SpecInfo,
};
use generate_workflow::generate_workflow;
use http::Method;
use oas3::spec::{Operation, PathItem};

use self::input_map::create_input_map;

pub fn generate_workflows(workflow_spec_infos: Vec<SpecInfo>, re_exports: &mut ReExports) {
    let mut workflow_definition_names = vec![];
    let input_map = create_input_map();

    let operations: Vec<(&SpecInfo, String, PathItem, Method, &Operation)> = workflow_spec_infos
        .iter()
        .flat_map(|spec_info| -> Vec<(&SpecInfo, String, &PathItem)> {
            spec_info
                .spec
                .paths
                .iter()
                .map(
                    |(path_string, path_item)| -> (&SpecInfo, String, &PathItem) {
                        (spec_info, path_string.to_string(), path_item)
                    },
                )
                .collect()
        })
        .flat_map(
            |(spec_info, path_string, path_item)| -> Vec<(&SpecInfo, String, PathItem, Method, &Operation)> {
                path_item
                    .methods()
                    .into_iter()
                    .map(
                        |(method, operation)| -> (&SpecInfo, String, PathItem, Method, &Operation) {
                            (spec_info, path_string.to_string(), path_item.clone(), method, operation)
                        },
                    )
                    .collect()
            },
        )
        .collect();

    for (spec_info, path_string, path_item, method, operation) in operations {
        let request_parameters_from_spec =
            extract_request_parameters_from_spec(&path_item, operation, &spec_info.spec);

        let names = generate_workflow(
            request_parameters_from_spec,
            operation,
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
