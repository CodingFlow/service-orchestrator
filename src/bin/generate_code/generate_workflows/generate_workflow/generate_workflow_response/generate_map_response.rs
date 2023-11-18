mod create_function_signature;
mod create_query_destructure;
mod create_reply;
mod create_service_calls;

use crate::{
    generate_workflows::{
        generate_workflow::{build_view_data::WorkflowRequestSpec, variables::VariableAliases},
        input_map::InputMap,
    },
    traversal::NestedNode,
};
use codegen::{Function, Scope};
use create_function_signature::create_function_signature;
use create_query_destructure::create_query_destructure;
use create_reply::create_reply;
use create_service_calls::create_service_calls;

use super::generate_response_structure::ResponseWithStructName;

pub fn generate_map_response(
    status_code_struct_names: Vec<(String, NestedNode<ResponseWithStructName>)>,
    scope: &mut Scope,
    workflow_request_spec: WorkflowRequestSpec,
    query_struct_name: &str,
    input_map: &mut InputMap,
    workflow_name: String,
    variable_aliases: &mut VariableAliases,
) {
    let map_functions: Vec<Function> = status_code_struct_names
        .iter()
        .map(|status_code_struct_name_node| -> Function {
            map_function(
                status_code_struct_name_node.clone(),
                workflow_request_spec.clone(),
                query_struct_name,
                input_map,
                workflow_name.to_string(),
                scope,
                variable_aliases,
            )
        })
        .collect();

    for function in map_functions {
        scope.push_fn(function);
    }
}

fn map_function(
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    workflow_request_spec: WorkflowRequestSpec,
    query_struct_name: &str,
    input_map: &mut InputMap,
    workflow_name: String,
    scope: &mut Scope,
    variable_aliases: &mut VariableAliases,
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

    create_reply(
        &mut function,
        status_code_struct_name_node,
        workflow_request_spec.query,
        input_map,
        workflow_name,
    );

    function
}
