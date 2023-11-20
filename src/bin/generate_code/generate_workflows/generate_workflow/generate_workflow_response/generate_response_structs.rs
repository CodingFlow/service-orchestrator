use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_response_variables::ServiceResponseAlias;
use crate::parse_specs::parse_schema::to_string_schema;
use crate::traversal::traverse_nested_node;
use crate::traversal::NestedNode;
use codegen::Field;
use codegen::Scope;
use codegen::Struct;

pub fn generate_response_structs(
    response_specs: Vec<NestedNode<ServiceResponseAlias>>,
    scope: &mut Scope,
) {
    let status_code_structs: Vec<(String, Vec<Struct>)> = response_specs
        .iter()
        .flat_map(|nested_response_alias_node| {
            // TODO: handle multiple status codes
            create_structs(nested_response_alias_node.clone())
        })
        .collect();

    for (status_code, structs) in status_code_structs {
        for new_struct in structs {
            scope.push_struct(new_struct);
        }
    }
}

fn create_structs(
    nested_response_alias_node: NestedNode<ServiceResponseAlias>,
) -> Vec<(String, Vec<Struct>)> {
    // TODO: handle more than one status code

    vec![nested_process(nested_response_alias_node)]
}

fn nested_process(
    nested_response_alias_node: NestedNode<ServiceResponseAlias>,
) -> (String, Vec<Struct>) {
    let structs = &mut vec![];

    traverse_nested_node(
        nested_response_alias_node.clone(),
        process_parent,
        process_child,
        process_after_children,
        structs,
    );

    ("200".to_string(), structs.to_vec())
}

fn process_parent<'a>(
    parent_node: NestedNode<ServiceResponseAlias>,
    _: &'a mut Vec<Struct>,
) -> Option<Struct> {
    match parent_node.children.is_some() {
        true => {
            let struct_name = parent_node.current.variable_alias;
            let mut new_struct = Struct::new(&struct_name);

            new_struct
                .derive("Serialize")
                .derive("Deserialize")
                .derive("Clone")
                .derive("Debug");

            Some(new_struct)
        }
        false => None,
    }
}

fn process_child<'a>(
    child_node: NestedNode<ServiceResponseAlias>,
    parent_struct: &'a mut Option<Struct>,
    _: &'a mut Vec<Struct>,
) {
    if let Some(parent_struct) = parent_struct {
        let field = Field::new(
            &child_node.current.name.clone().unwrap(),
            to_string_schema(
                child_node.current.schema_type,
                Some(child_node.current.variable_alias),
            ),
        );

        parent_struct.push_field(field.clone());
    };
}

fn process_after_children(parent_struct: Option<Struct>, struct_accumulator: &mut Vec<Struct>) {
    if let Some(parent_struct) = parent_struct {
        struct_accumulator.push(parent_struct);
    }
}
