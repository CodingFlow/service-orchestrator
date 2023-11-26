use crate::traversal::{traverse_nested_node, NestedNode};
use codegen::Function;
use oas3::spec::SchemaType;

#[derive(Debug, Clone)]
pub struct BodyPropertyAlias {
    /// name of property from open api spec. Optional because top level
    /// node does not have a name.
    pub name: Option<String>,
    pub variable_alias: String,
    pub schema_type: SchemaType,
    pub alias_type: AliasType,
}

#[derive(Debug, Clone)]
pub enum AliasType {
    Struct,
    Field,
}

pub fn generate_body_variables(
    mut function: &mut Function,
    response_aliases: &NestedNode<BodyPropertyAlias>,
    should_clone_strings: bool,
) {
    function.line(format!("{} {{", response_aliases.current.variable_alias));

    traverse_nested_node(
        response_aliases.clone(),
        |parent_node: NestedNode<BodyPropertyAlias>,
         (function, should_clone_strings): &mut (&mut &mut Function, bool)| {
            if let Some(_) = parent_node.current.name.clone() {
                let line = match parent_node.children.is_some() {
                    true => {
                        format!(
                            "{}: {} {{",
                            parent_node.current.name.clone().unwrap(),
                            parent_node.current.variable_alias
                        )
                    }
                    false => {
                        let clone = match should_clone_strings {
                            true => match parent_node.current.schema_type {
                                SchemaType::String => ".clone()".to_string(),
                                _ => String::new(),
                            },
                            false => String::new(),
                        };

                        format!(
                            "{}: {}{},",
                            parent_node.current.name.clone().unwrap(),
                            parent_node.current.variable_alias,
                            clone
                        )
                    }
                };

                function.line(line);
            }

            parent_node.current.name
        },
        |_, _, _| {},
        |node_name, (function, _)| {
            match node_name {
                Some(_) => function.line("},"),
                None => function.line("}"), // only for top level
            };
        },
        &mut (&mut function, should_clone_strings),
    );
}
