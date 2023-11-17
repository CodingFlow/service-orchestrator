use crate::parse_specs::parse_schema::to_string_schema;
use crate::parse_specs::parse_schema::ParsedSchema;
use crate::parse_specs::ResponseSpec;
use crate::traversal::traverse_nested_node;
use crate::traversal::traverse_nested_type;
use crate::traversal::NestedNode;
use codegen::Field;
use codegen::Scope;
use codegen::Struct;
use std::slice;

pub fn generate_response_structs(
    struct_names: Vec<String>,
    response_specs: Vec<Vec<ResponseSpec>>,
    scope: &mut Scope,
) {
    let struct_names_iter = &mut struct_names.iter();
    let nested_structs: Vec<(String, NestedNode<Option<Struct>>)> = response_specs
        .iter()
        .flat_map(|response_spec| {
            // TODO: handle multiple status codes
            create_structs(response_spec.clone(), struct_names_iter)
        })
        .collect();

    for (status_code, nested_struct) in nested_structs {
        traverse_nested_node(
            nested_struct,
            |nested_struct, scope| {
                if let Some(struct_name) = nested_struct.current {
                    scope.push_struct(struct_name);
                }
            },
            |_, _, _| {},
            scope,
        );
    }
}

fn create_structs(
    response_specs: Vec<ResponseSpec>,
    mut struct_names_iter: &mut slice::Iter<'_, std::string::String>,
) -> Vec<(String, NestedNode<Option<Struct>>)> {
    response_specs
        .iter()
        .map(|response_spec| nested_process(response_spec, &mut struct_names_iter))
        .collect()
}

fn nested_process(
    parent: &ResponseSpec,
    struct_names_iter: &mut slice::Iter<'_, String>,
) -> (String, NestedNode<Option<Struct>>) {
    let nested_node = traverse_nested_type(
        parent.body.clone(),
        process_parent,
        process_child,
        get_children,
        struct_names_iter,
    );

    (parent.status_code.to_string(), nested_node)
}

fn process_parent(
    parent_schema: ParsedSchema,
    struct_names: &mut slice::Iter<'_, std::string::String>,
) -> Option<Struct> {
    match parent_schema.properties.is_some() {
        true => {
            let struct_name = struct_names.next().unwrap();
            let mut new_struct = Struct::new(struct_name);

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
    child_schema: ParsedSchema,
    parent_struct: &'a mut Option<Struct>,
    _: &mut slice::Iter<'_, std::string::String>,
) {
    if let Some(parent_struct) = parent_struct {
        let field = Field::new(
            &child_schema.name.clone().unwrap(),
            to_string_schema(child_schema.schema_type, None),
        );

        parent_struct.push_field(field.clone());
    };
}

fn get_children(schema: ParsedSchema) -> Option<Vec<ParsedSchema>> {
    schema.properties
}
