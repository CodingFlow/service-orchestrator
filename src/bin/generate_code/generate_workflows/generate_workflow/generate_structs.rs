use crate::parse_specs::parse_schema::to_string_schema;
use codegen::Field;
use crate::traversal::traverse_nested_node;
use codegen::Struct;
use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_body_variables::BodyPropertyAlias;
use crate::traversal::NestedNode;

pub fn create_structs(nested_response_alias_node: NestedNode<BodyPropertyAlias>) -> Vec<Struct> {
    let structs = &mut vec![];

    traverse_nested_node(
        nested_response_alias_node.clone(),
        process_parent,
        process_child,
        process_after_children,
        structs,
    );

    structs.to_vec()
}

fn process_parent<'a>(
    parent_node: NestedNode<BodyPropertyAlias>,
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
                .derive("Debug")
                .vis("pub");

            Some(new_struct)
        }
        false => None,
    }
}

fn process_child<'a>(
    child_node: NestedNode<BodyPropertyAlias>,
    parent_struct: &'a mut Option<Struct>,
    _: &'a mut Vec<Struct>,
) {
    if let Some(parent_struct) = parent_struct {
        let field = Field::new(
            &format!("pub {}", &child_node.current.name.clone().unwrap()),
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
