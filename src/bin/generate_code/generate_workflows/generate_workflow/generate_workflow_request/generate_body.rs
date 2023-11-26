use crate::{
    generate_workflows::generate_workflow::build_service_call_view_data::generate_body_variables::BodyPropertyAlias,
    traversal::NestedNode,
};
use codegen::Function;

pub fn generate_body(function: &mut Function, body: Option<NestedNode<BodyPropertyAlias>>) {
    if let Some(_) = body {
        function.line(".and(warp::body::json())");
    }
}
