use codegen::{Field, Scope, Struct};

use crate::{
    spec_parsing::{to_string_schema, ParsedSchema},
    traversal::{traverse_nested_type, NestedNode},
};

pub fn generate_response_structure(
    response_values: Vec<(String, ParsedSchema)>,
    scope: &mut Scope,
) -> Vec<(String, NestedNode<Option<String>>)> {
    let responses = create_structs(response_values);

    for (_, node) in responses.clone() {
        traverse_nested_type(
            node,
            |current_node, scope| -> () {
                if let Some((_, structure)) = current_node.current {
                    scope.push_struct(structure);
                }
            },
            |child_node, parent_result, scope| {},
            |current_node| -> Option<Vec<NestedNode<Option<(String, Struct)>>>> {
                current_node.children
            },
            &mut *scope,
            false,
        );
    }

    responses
        .iter()
        .map(
            |(status_code, node)| -> (String, NestedNode<Option<String>>) {
                let new_node = traverse_nested_type(
                    node.clone(),
                    |current_node, _| -> Option<String> {
                        match current_node.current {
                            Some((struct_name, _)) => Some(struct_name),
                            None => None,
                        }
                    },
                    |child_node, parent_result, _| {},
                    |current_node| -> Option<Vec<NestedNode<Option<(String, Struct)>>>> {
                        current_node.children
                    },
                    &mut (),
                    false,
                );

                (status_code.to_string(), new_node)
            },
        )
        .collect()
}

fn create_structs(
    response_values: Vec<(String, ParsedSchema)>,
) -> Vec<(String, NestedNode<Option<(String, Struct)>>)> {
    response_values.iter().map(nested_process).collect()
}

fn nested_process(
    parent: &(String, ParsedSchema),
) -> (String, NestedNode<Option<(String, Struct)>>) {
    let (status_code, schema) = parent;

    let nested_node = traverse_nested_type(
        schema.clone(),
        process_parent,
        process_child,
        get_children,
        &mut (),
        false,
    );

    (status_code.to_string(), nested_node)
}

fn process_parent(parent_schema: ParsedSchema, _: &mut ()) -> Option<(String, Struct)> {
    match parent_schema.properties.is_some() {
        true => {
            let struct_name = &format!(
                "WorkflowResponse_{}",
                parent_schema
                    .name
                    .clone()
                    .get_or_insert("top_level".to_string())
            );
            let mut new_struct = Struct::new(struct_name);

            new_struct.derive("Serialize").derive("Deserialize");

            Some((struct_name.to_string(), new_struct))
        }
        false => None,
    }
}

fn process_child<'a>(
    child_schema: ParsedSchema,
    parent_result: &'a mut Option<(String, Struct)>,
    _: &mut (),
) {
    if let Some((_, ref mut parent_struct)) = parent_result {
        let field = Field::new(
            &child_schema.name.clone().unwrap(),
            to_string_schema(
                child_schema.schema_type,
                Some(format!(
                    "WorkflowResponse_{}",
                    child_schema.name.clone().unwrap()
                )),
            ),
        );

        parent_struct.push_field(field.clone());
    };
}

fn get_children(schema: ParsedSchema) -> Option<Vec<ParsedSchema>> {
    schema.properties
}
