mod generate_futures;
mod generate_imports;
mod generate_response_handling;
mod generate_streams;

use super::build_service_operation_lookup_map::ServiceCodeGenerationInfo;
use super::build_service_operation_lookup_map::ServiceResponseAlias;
use super::build_workflow_response_lookup_map::WorkflowResponseCodeGenerationInfo;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::traversal::traverse_nested_node;
use crate::traversal::NestedNode;
use codegen::Function;
use codegen::Scope;
use generate_futures::generate_futures;
use generate_imports::generate_imports;
use generate_response_handling::generate_response_handling;
use generate_streams::generate_streams;
use std::collections::BTreeMap;

pub fn generate_service_calls(
    scope: &mut Scope,
    function: &mut Function,
    generation_infos: (
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
    workflow_response_code_generation_info: WorkflowResponseCodeGenerationInfo,
    variable_aliases: &mut VariableAliases,
) {
    generate_imports(scope);

    generate_futures(function, &generation_infos, variable_aliases);

    generate_streams(function, generation_infos.1.to_vec());

    generate_response_handling(function, workflow_response_code_generation_info);
}

pub fn generate_response_variables(
    mut function: &mut Function,
    response_aliases: &NestedNode<ServiceResponseAlias>,
) {
    function.line(format!(
        "let {} {{",
        response_aliases.current.variable_alias
    ));

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
