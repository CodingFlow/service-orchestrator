mod create_function_signature;
mod create_query_destructure;
mod create_service_calls;
mod generate_reply;

use crate::{
    generate_workflows::{
        generate_workflow::{build_view_data::WorkflowRequestSpec, variables::VariableAliases},
        input_map::InputMap,
    },
    parse_specs::OperationSpec,
};
use codegen::{Function, Scope};
use create_function_signature::create_function_signature;
use create_query_destructure::create_query_destructure;
use create_service_calls::create_service_calls;
use generate_reply::generate_reply;

use super::{
    create_workflow_response_aliases::create_workflow_response_aliases,
    generate_response_structs::generate_response_structs,
};

pub fn generate_map_response(
    workflow_spec: OperationSpec,
    scope: &mut Scope,
    workflow_request_spec: WorkflowRequestSpec,
    query_struct_name: &str,
    input_map: &mut InputMap,
    workflow_name: String,
    variable_aliases: &mut VariableAliases,
) {
    let function = map_function(
        workflow_spec,
        workflow_request_spec.clone(),
        query_struct_name,
        input_map,
        workflow_name.to_string(),
        scope,
        variable_aliases,
    );

    scope.push_fn(function);
}

fn map_function(
    workflow_spec: OperationSpec,
    workflow_request_spec: WorkflowRequestSpec,
    query_struct_name: &str,
    input_map: &mut InputMap,
    workflow_name: String,
    scope: &mut Scope,
    mut variable_aliases: &mut VariableAliases,
) -> Function {
    let mut function = Function::new("map_response");

    create_function_signature(&mut function, workflow_request_spec.path, query_struct_name);

    create_query_destructure(
        &mut function,
        query_struct_name,
        workflow_request_spec.query.to_vec(),
    );

    create_service_calls(
        &mut function,
        input_map,
        workflow_name.to_string(),
        scope,
        variable_aliases,
    );

    let response_aliases = create_workflow_response_aliases(
        vec![workflow_spec].iter(),
        input_map,
        &mut variable_aliases,
        workflow_name.to_string(),
    );

    generate_response_structs(response_aliases.to_vec(), scope);

    generate_reply(&mut function, response_aliases);

    function
}
