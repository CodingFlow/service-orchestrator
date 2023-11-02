use codegen::{Field, Scope, Struct};

use crate::{
    spec_parsing::{to_string_schema, ParsedSchema},
    traversal::{traverse_nested_type, NestedNode},
};

pub fn generate_response_structure(
    response_values: Vec<(String, ParsedSchema)>,
    scope: &mut Scope,
) -> Vec<NestedNode<(String, String)>> {
    let responses: Vec<NestedNode<(String, String, Struct)>> = create_structs(response_values);

    for nested_node in responses.clone() {
        traverse_nested_type(
            nested_node,
            |current_node, scope| -> () {
                let (_, _, structure) = current_node.current;

                scope.push_struct(structure);
            },
            |child_node, parent_result, scope| {},
            |current_node| -> Option<Vec<NestedNode<(String, String, Struct)>>> {
                current_node.children
            },
            &mut *scope,
            false,
        );
    }

    responses
        .iter()
        .map(|nested_node| -> NestedNode<(String, String)> {
            traverse_nested_type(
                nested_node.clone(),
                |current_node, _| -> (String, String) {
                    let (status_code, struct_name, _) = current_node.current;

                    (status_code, struct_name)
                },
                |child_node, parent_result, _| {},
                |current_node| -> Option<Vec<NestedNode<(String, String, Struct)>>> {
                    current_node.children
                },
                &mut (),
                false,
            )
        })
        .collect()
}

fn create_structs(
    response_values: Vec<(String, ParsedSchema)>,
) -> Vec<NestedNode<(String, String, Struct)>> {
    response_values.iter().map(nested_process).collect()
}

fn nested_process(parent: &(String, ParsedSchema)) -> NestedNode<(String, String, Struct)> {
    traverse_nested_type(
        parent.clone(),
        process_parent,
        process_child,
        |(status_code, schema): (String, ParsedSchema)| -> Option<Vec<(String, ParsedSchema)>> {
            if let Some(schema_properties) = schema.properties {
                Some(
                    schema_properties
                        .iter()
                        .map(|child_schema| -> (String, ParsedSchema) {
                            (status_code.to_string(), child_schema.clone())
                        })
                        .collect(),
                )
            } else {
                None
            }
        },
        &mut (),
        true,
    )
}

fn process_parent(
    (status_code, parent_schema): (String, ParsedSchema),
    _: &mut (),
) -> (String, String, Struct) {
    let struct_name = &format!(
        "WorkflowResponse_{}_{}",
        status_code,
        parent_schema
            .name
            .clone()
            .get_or_insert("top_level".to_string())
    );
    let mut new_struct = Struct::new(struct_name);

    new_struct.derive("Serialize").derive("Deserialize");

    (status_code.to_string(), struct_name.to_string(), new_struct)
}

fn process_child<'a>(
    (child_status_code, child_schema): (String, ParsedSchema),
    (status_code, parent_struct_name, ref mut parent_struct): &'a mut (String, String, Struct),
    _: &mut (),
) {
    let field = Field::new(
        &child_schema.name.clone().unwrap(),
        to_string_schema(
            child_schema.schema_type,
            Some(format!(
                "WorkflowResponse_{}_{}",
                status_code,
                child_schema.name.clone().unwrap()
            )),
        ),
    );

    parent_struct.push_field(field.clone());
}
