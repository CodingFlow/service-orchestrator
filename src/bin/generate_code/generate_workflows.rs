mod generate_workflow;
mod input_map;

use crate::{
    generate_create_filter::generate_create_filter, generate_re_exports::ReExports,
    parse_workflow_specs::SpecInfo,
};
use generate_workflow::generate_workflow;

use self::input_map::create_input_map;

pub fn generate_workflows(workflow_spec_infos: Vec<SpecInfo>, re_exports: &mut ReExports) {
    let mut workflow_definition_names = vec![];
    let input_map = create_input_map();

    for spec_info in workflow_spec_infos {
        for (path_string, path_item) in &spec_info.spec.paths {
            for (method, operation) in path_item.methods() {
                let names = generate_workflow(
                    &path_item,
                    operation,
                    &spec_info,
                    method,
                    &path_string,
                    &input_map,
                    re_exports,
                );

                workflow_definition_names.push(names);
            }
        }
    }

    generate_create_filter(workflow_definition_names, re_exports);
}
