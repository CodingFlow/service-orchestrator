use crate::{traversal::NestedNode, generate_workflows::generate_workflow::build_service_call_view_data::generate_body_variables::{generate_body_variables, BodyPropertyAlias}};
use codegen::Function;

pub fn generate_request_body_destructure(
    function: &mut Function,
    body: Option<NestedNode<BodyPropertyAlias>>,
    body_local_variable: String,
) {
    if let Some(body) = body {
        function.line("let ");

        generate_body_variables(function, &body, false);

        function.line(format!("= {};", body_local_variable));
    }
}
