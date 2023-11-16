use crate::generate_workflows::input_map::Variable;
use crate::traversal::NestedNode;
use crate::traversal::traverse_nested_node;
use crate::generate_workflows::generate_workflow::generate_workflow_response::generate_map_response::create_service_calls::variables::VariableAliases;
use codegen::Function;
use super::super::super::build_loopkup_map::ServiceCodeGenerationInfo;
use std::collections::BTreeMap;

pub fn generate_signature_and_dependencies_variables(
    number_depending: usize,
    dependencies_service_and_operation_names: &Vec<(String, String)>,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    function: &mut Function,
    future_variable_name: &String,
    variable_aliases: &mut VariableAliases,
) {
    match number_depending {
        number if number > 1 => {
            {
                let future_variable_name = future_variable_name;
                let (
                    depending_operation_aliases,
                    depending_future_variable_names,
                    formatted_depending_operation_aliases,
                ) = create_future_signature_data_with_dependencies(
                    dependencies_service_and_operation_names,
                    variable_aliases,
                    generation_infos,
                );

                function.line(format!(
                    "let {} = async {{ tokio::join!({}) }}.then(|({})| async {{",
                    future_variable_name,
                    depending_future_variable_names,
                    formatted_depending_operation_aliases,
                ));

                generate_future_dependencies_variables(
                    dependencies_service_and_operation_names,
                    generation_infos,
                    function,
                    depending_operation_aliases,
                );
            };
        }
        number if number == 1 => {
            let (
                depending_operation_aliases,
                depending_future_variable_names,
                formatted_depending_operation_aliases,
            ) = create_future_signature_data_with_dependencies(
                dependencies_service_and_operation_names,
                variable_aliases,
                generation_infos,
            );

            function.line(format!(
                "let {} = {}.then(|{}| async {{",
                future_variable_name,
                depending_future_variable_names,
                formatted_depending_operation_aliases,
            ));

            generate_future_dependencies_variables(
                dependencies_service_and_operation_names,
                generation_infos,
                function,
                depending_operation_aliases,
            );
        }
        number if number == 0 => {
            function.line(format!("let {} = async {{", future_variable_name));
        }
        _ => {}
    };
}

pub fn create_future_signature_data_with_dependencies(
    depending_service_and_operation_names: &Vec<(String, String)>,
    variable_aliases: &mut VariableAliases,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
) -> (Vec<String>, String, String) {
    let depending_operation_aliases = depending_service_and_operation_names
        .iter()
        .map(|_| variable_aliases.create_alias())
        .collect::<Vec<String>>();

    let depending_future_variable_names = depending_service_and_operation_names
        .iter()
        .map(|service_and_operation_name| generation_infos.get(service_and_operation_name).unwrap())
        .map(|info| info.future_variable_name.to_string())
        .map(|var_name| format!("{}.clone()", var_name))
        .collect::<Vec<String>>()
        .join(",");

    let formatted_depending_operation_aliases = depending_operation_aliases.join(",");
    (
        depending_operation_aliases,
        depending_future_variable_names,
        formatted_depending_operation_aliases,
    )
}

pub fn generate_future_dependencies_variables(
    depending_service_and_operation_names: &Vec<(String, String)>,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    mut function: &mut Function,
    depending_operation_aliases: Vec<String>,
) {
    let mut depending_operation_alias_iter = depending_operation_aliases.iter();

    for depending_service_and_operation in depending_service_and_operation_names {
        let depending_service_code_generation_info = generation_infos
            .get(depending_service_and_operation)
            .unwrap();
        let depending_service_struct_name =
            &depending_service_code_generation_info.response_struct_name;

        function.line(format!("let {} {{", depending_service_struct_name));
        traverse_nested_node(
            depending_service_code_generation_info
                .response_aliases
                .clone(),
            |parent_node: NestedNode<Option<Variable>>,
             (function, service_struct_name): &mut (&mut &mut Function, &String)| {
                // TODO: Need to add closing curly brace for nested response objects
                // if parent_node.children.is_some() {
                //     function.line("},");
                // }

                if let Some(node) = parent_node.current {
                    match parent_node.children.is_some() {
                        true => {
                            function.line(format!(
                                "{}: {} {{",
                                node.original_name, service_struct_name
                            ));
                        }
                        false => {
                            function.line(format!("{}: {},", node.original_name, node.alias));
                        }
                    };
                }
            },
            |child_node, _, (function, service_struct_name)| {},
            &mut (&mut function, depending_service_struct_name),
        );

        let depending_operation_alias = depending_operation_alias_iter.next().unwrap();

        function.line(format!("}} = {}.unwrap();", depending_operation_alias));
    }
}
