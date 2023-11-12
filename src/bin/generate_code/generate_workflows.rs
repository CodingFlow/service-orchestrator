mod add_variable_aliases_to_request_parameters;
mod generate_workflow;
mod input_map;

use crate::{
    generate_create_filter::generate_create_filter, generate_re_exports::ReExports,
    parse_specs::OperationSpec,
};

use add_variable_aliases_to_request_parameters::add_variable_aliases_to_request_parameters;
use generate_workflow::generate_workflow;

use self::input_map::create_input_map;

pub fn generate_workflows(workflow_spec_infos: Vec<OperationSpec>, re_exports: &mut ReExports) {
    let mut workflow_definition_names = vec![];
    let mut input_map = create_input_map();

    for operation_spec in workflow_spec_infos {
        let workflow_operation_spec =
            add_variable_aliases_to_request_parameters(operation_spec, &mut input_map);

        let names = generate_workflow(workflow_operation_spec, &input_map, re_exports);

        workflow_definition_names.push(names);
    }

    generate_create_filter(workflow_definition_names, re_exports);
}
