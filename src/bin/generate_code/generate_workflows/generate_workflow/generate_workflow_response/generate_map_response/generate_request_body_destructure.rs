use crate::{traversal::NestedNode, generate_workflows::generate_workflow::build_service_call_view_data::generate_response_variables::{generate_response_variables, ResponseAlias}};
use codegen::Function;

pub fn generate_request_body_destructure(
    function: &mut Function,
    body: Option<NestedNode<ResponseAlias>>,
    body_local_variable: String,
) {
    if let Some(body) = body {
        function.line("let ");

        generate_response_variables(function, &body);

        function.line(format!("= {};", body_local_variable));
    }
}
