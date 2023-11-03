use codegen::{Field, Scope, Struct};

use crate::{
    spec_parsing::{to_string_schema, ParsedSchema},
    traversal::{traverse_nested_type, NestedNode},
};

pub fn generate_response_structure(
    response_values: Vec<(String, ParsedSchema)>,
    scope: &mut Scope,
) -> Vec<(String, NestedNode<String>)> {
    let responses = create_structs(response_values);

    for (_, node) in responses.clone() {
        traverse_nested_type(
            node,
            |current_node, scope| -> () {
                let (_, structure) = current_node.current;

                scope.push_struct(structure);
            },
            |child_node, parent_result, scope| {},
            |current_node| -> Option<Vec<NestedNode<(String, Struct)>>> { current_node.children },
            &mut *scope,
            false,
        );
    }

    responses
        .iter()
        .map(|(status_code, node)| -> (String, NestedNode<String>) {
            let new_node = traverse_nested_type(
                node.clone(),
                |current_node, _| -> String {
                    let (struct_name, _) = current_node.current;

                    struct_name
                },
                |child_node, parent_result, _| {},
                |current_node| -> Option<Vec<NestedNode<(String, Struct)>>> {
                    current_node.children
                },
                &mut (),
                false,
            );

            (status_code.to_string(), new_node)
        })
        .collect()
}

fn create_structs(
    response_values: Vec<(String, ParsedSchema)>,
) -> Vec<(String, NestedNode<(String, Struct)>)> {
    response_values.iter().map(nested_process).collect()
}

fn nested_process(parent: &(String, ParsedSchema)) -> (String, NestedNode<(String, Struct)>) {
    let (status_code, schema) = parent;

    let nested_node = traverse_nested_type(
        schema.clone(),
        process_parent,
        process_child,
        |schema| -> Option<Vec<ParsedSchema>> {
            if let Some(schema_properties) = schema.properties {
                Some(
                    schema_properties
                        .iter()
                        .map(|child_schema| -> ParsedSchema { child_schema.clone() })
                        .collect(),
                )
            } else {
                None
            }
        },
        &mut (),
        true,
    );

    (status_code.to_string(), nested_node)
}

fn process_parent(parent_schema: ParsedSchema, _: &mut ()) -> (String, Struct) {
    let struct_name = &format!(
        "WorkflowResponse_{}",
        parent_schema
            .name
            .clone()
            .get_or_insert("top_level".to_string())
    );
    let mut new_struct = Struct::new(struct_name);

    new_struct.derive("Serialize").derive("Deserialize");

    (struct_name.to_string(), new_struct)
}

fn process_child<'a>(
    child_schema: ParsedSchema,
    (parent_struct_name, ref mut parent_struct): &'a mut (String, Struct),
    _: &mut (),
) {
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
}
