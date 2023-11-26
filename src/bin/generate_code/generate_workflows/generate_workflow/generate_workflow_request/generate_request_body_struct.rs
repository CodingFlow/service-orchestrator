use super::super::build_service_call_view_data::generate_body_variables::BodyPropertyAlias;
use crate::{
    generate_workflows::generate_workflow::generate_structs::generate_structs,
    traversal::NestedNode,
};
use codegen::Scope;

pub fn generate_request_body_struct(
    scope: &mut Scope,
    body: Option<NestedNode<BodyPropertyAlias>>,
) {
    if let Some(body) = body {
        let structs = generate_structs(body);

        for new_struct in structs {
            scope.push_struct(new_struct);
        }
    }
}
