use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_body_variables::BodyPropertyAlias;
use crate::generate_workflows::generate_workflow::create_structs::create_structs;
use crate::traversal::NestedNode;
use codegen::Scope;
use codegen::Struct;

pub fn generate_response_structs(
    response_specs: Vec<NestedNode<BodyPropertyAlias>>,
    scope: &mut Scope,
) {
    let status_code_structs: Vec<(String, Vec<Struct>)> = response_specs
        .iter()
        .flat_map(|nested_response_alias_node| {
            // TODO: handle multiple status codes
            create_response_structs(nested_response_alias_node.clone())
        })
        .collect();

    for (status_code, structs) in status_code_structs {
        for new_struct in structs {
            scope.push_struct(new_struct);
        }
    }
}

fn create_response_structs(
    nested_response_alias_node: NestedNode<BodyPropertyAlias>,
) -> Vec<(String, Vec<Struct>)> {
    // TODO: handle more than one status code

    vec![nested_process(nested_response_alias_node)]
}

fn nested_process(
    nested_response_alias_node: NestedNode<BodyPropertyAlias>,
) -> (String, Vec<Struct>) {
    let structs = create_structs(nested_response_alias_node);

    ("200".to_string(), structs.to_vec())
}
