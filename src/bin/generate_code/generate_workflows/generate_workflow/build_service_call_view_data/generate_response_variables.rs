use codegen::Function;
use oas3::spec::SchemaType;

use crate::traversal::{traverse_nested_node, NestedNode};

#[derive(Debug, Clone)]
pub struct ServiceResponseAlias {
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

pub fn generate_response_variables(
    mut function: &mut Function,
    response_aliases: &NestedNode<ServiceResponseAlias>,
) {
    function.line(format!("{} {{", response_aliases.current.variable_alias));

    traverse_nested_node(
        response_aliases.clone(),
        |parent_node: NestedNode<ServiceResponseAlias>, function: &mut &mut Function| {
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
                        format!(
                            "{}: {},",
                            parent_node.current.name.clone().unwrap(),
                            parent_node.current.variable_alias
                        )
                    }
                };

                function.line(line);
            }

            parent_node.current.name
        },
        |_, _, _| {},
        |node_name, function| {
            match node_name {
                Some(_) => function.line("},"),
                None => function.line("}"), // only for top level
            };
        },
        &mut function,
    );
}
