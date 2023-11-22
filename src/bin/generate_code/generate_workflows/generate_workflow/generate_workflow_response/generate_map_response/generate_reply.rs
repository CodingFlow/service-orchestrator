use codegen::Function;

use crate::{generate_workflows::generate_workflow::build_service_call_view_data::generate_response_variables::{ServiceResponseAlias, generate_response_variables}, traversal::NestedNode};

pub fn generate_reply(
    function: &mut Function,
    response_aliases: Vec<NestedNode<ServiceResponseAlias>>,
) {
    // TODO: handle more than one status code
    let response_alias = response_aliases.first().unwrap();

    function.line("Ok(reply::json(&");

    generate_response_variables(function, &response_alias.clone());

    function.line("))");
}