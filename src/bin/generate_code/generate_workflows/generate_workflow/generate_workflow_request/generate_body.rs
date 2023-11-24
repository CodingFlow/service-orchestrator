use super::super::build_service_call_view_data::generate_response_variables::ResponseAlias;
use crate::traversal::NestedNode;
use codegen::Function;

pub fn generate_body(function: &mut Function, body: Option<NestedNode<ResponseAlias>>) {
    if let Some(_) = body {
        function.line(".and(warp::body::json())");
    }
}
