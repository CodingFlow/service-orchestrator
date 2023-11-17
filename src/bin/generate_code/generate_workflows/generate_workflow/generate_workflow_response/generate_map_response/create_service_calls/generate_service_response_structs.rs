use crate::{
    generate_workflows::generate_workflow::generate_workflow_response::generate_response_structs,
    parse_specs::{OperationSpec, ResponseSpec},
};
use codegen::Scope;
use generate_response_structs::generate_response_structs;

pub fn generate_service_response_structs(
    scope: &mut Scope,
    operation_specs: Vec<OperationSpec>,
    struct_names: Vec<String>,
) {
    let response_specs: Vec<Vec<ResponseSpec>> = operation_specs
        .iter()
        .map(|operation_spec| operation_spec.response_specs.clone())
        .collect();

    generate_response_structs(struct_names, response_specs, scope);
}
