mod generate_function_signature;
mod generate_query_destructure;
mod generate_reply;
mod generate_service_calls;

use super::generate_response_structs::generate_response_structs;
use crate::{
    generate_workflows::generate_workflow::{
        build_service_call_view_data::{
            generate_response_variables::ServiceResponseAlias, ServiceCallGenerationInfo,
        },
        build_view_data::WorkflowRequestSpec,
        variables::VariableAliases,
    },
    traversal::NestedNode,
};
use codegen::{Function, Scope};
use generate_function_signature::generate_function_signature;
use generate_query_destructure::generate_query_destructure;
use generate_reply::generate_reply;
use generate_service_calls::generate_service_calls;

pub fn generate_map_response(
    scope: &mut Scope,
    workflow_request_spec: WorkflowRequestSpec,
    query_struct_name: &str,
    variable_aliases: &mut VariableAliases,
    service_call_view_data: ServiceCallGenerationInfo,
    response_aliases: Vec<NestedNode<ServiceResponseAlias>>,
) {
    let function = map_function(
        workflow_request_spec.clone(),
        query_struct_name,
        scope,
        variable_aliases,
        service_call_view_data,
        response_aliases,
    );

    scope.push_fn(function);
}

fn map_function(
    workflow_request_spec: WorkflowRequestSpec,
    query_struct_name: &str,
    scope: &mut Scope,
    variable_aliases: &mut VariableAliases,
    service_call_view_data: ServiceCallGenerationInfo,
    response_aliases: Vec<NestedNode<ServiceResponseAlias>>,
) -> Function {
    let mut function = Function::new("map_response");

    generate_function_signature(&mut function, workflow_request_spec.path, query_struct_name);

    generate_query_destructure(
        &mut function,
        query_struct_name,
        workflow_request_spec.query.to_vec(),
    );

    generate_service_calls(
        &mut function,
        scope,
        variable_aliases,
        service_call_view_data,
    );

    generate_response_structs(response_aliases.to_vec(), scope);

    generate_reply(&mut function, response_aliases);

    function
}
