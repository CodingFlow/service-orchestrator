mod generate_workflow;
mod input_map;

use crate::{
    generate_create_filter::generate_create_filter, generate_re_exports::ReExports,
    parse_specs::OperationSpec,
};

use self::input_map::create_input_map;
use generate_workflow::generate_workflow;

pub fn generate_workflows(workflow_specs: Vec<OperationSpec>, re_exports: &mut ReExports) {
    let mut workflow_definition_names = vec![];
    let mut input_map = create_input_map();

    for operation_spec in workflow_specs {
        let names = generate_workflow(operation_spec, &mut input_map, re_exports);

        workflow_definition_names.push(names);
    }

    generate_create_filter(workflow_definition_names, re_exports);
}
