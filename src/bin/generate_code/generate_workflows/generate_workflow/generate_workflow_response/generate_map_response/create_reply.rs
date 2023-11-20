use codegen::Function;

use crate::{
    generate_workflows::{
        generate_workflow::{
            build_view_data::RequestParameter,
            generate_workflow_response::generate_response_variables::{
                generate_response_variables, ServiceResponseAlias,
            },
        },
        input_map::InputMap,
    },
    traversal::NestedNode,
};

pub fn create_reply(
    function: &mut Function,
    response_aliases: Vec<NestedNode<ServiceResponseAlias>>,
) {
    // TODO: handle more than one status code
    let response_alias = response_aliases.first().unwrap();

    function.line("Ok(reply::json(&");

    create_properties(response_alias.clone(), function);

    function.line("))");
}

fn create_properties(response_alias: NestedNode<ServiceResponseAlias>, function: &mut Function) {
    // TODO: Handle different status codes.

    generate_response_variables(function, &response_alias);
}
