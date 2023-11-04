use codegen::{Field, Scope, Struct};

use crate::{
    spec_parsing::{to_string_schema, ParsedSchema},
    traversal::{traverse_nested_type, NestedNode},
};

#[derive(Clone, Debug)]
pub struct ResponseWithStructName {
    pub struct_name: Option<String>,
    pub schema: ParsedSchema,
}

pub fn generate_response_structure(
    response_values: Vec<(String, ParsedSchema)>,
    scope: &mut Scope,
) -> Vec<(String, NestedNode<ResponseWithStructName>)> {
    let responses = create_structs(response_values);

    for (_, node) in responses.clone() {
        traverse_nested_type(
            node,
            |current_node, scope| -> () {
                let (_, structure) = current_node.current;
                if let Some(structure) = structure {
                    scope.push_struct(structure);
                }
            },
            |_, _, _| {},
            |current_node| -> Option<Vec<NestedNode<(ResponseWithStructName, Option<Struct>)>>> {
                current_node.children
            },
            &mut *scope,
            false,
        );
    }

    responses
        .iter()
        .map(
            |(status_code, node)| -> (String, NestedNode<ResponseWithStructName>) {
                let new_node = traverse_nested_type(
                    node.clone(),
                    |current_node, _| -> ResponseWithStructName {
                        let (response_with_struct_name, _) = current_node.current;

                        response_with_struct_name
                    },
                    |_, _, _| {},
                    |current_node| -> Option<Vec<NestedNode<(ResponseWithStructName, Option<Struct>)>>> {
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
) -> Vec<(String, NestedNode<(ResponseWithStructName, Option<Struct>)>)> {
    response_values.iter().map(nested_process).collect()
}

fn nested_process(
    parent: &(String, ParsedSchema),
) -> (String, NestedNode<(ResponseWithStructName, Option<Struct>)>) {
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

fn process_parent(
    parent_schema: ParsedSchema,
    _: &mut (),
) -> (ResponseWithStructName, Option<Struct>) {
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

            (
                ResponseWithStructName {
                    struct_name: Some(struct_name.to_string()),
                    schema: parent_schema,
                },
                Some(new_struct),
            )
        }
        false => (
            ResponseWithStructName {
                struct_name: None,
                schema: parent_schema,
            },
            None,
        ),
    }
}

fn process_child<'a>(
    child_schema: ParsedSchema,
    parent_result: &'a mut (ResponseWithStructName, Option<Struct>),
    _: &mut (),
) {
    let (_, parent_struct) = parent_result;

    if let Some(parent_struct) = parent_struct {
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
