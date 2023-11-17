mod generate_futures;

use crate::generate_workflows::input_map::Variable;
use crate::traversal::traverse_nested_node;
use crate::traversal::NestedNode;

use super::build_loopkup_map::ServiceCodeGenerationInfo;
use super::variables::VariableAliases;
use super::WorkflowResponseCodeGenerationInfo;
use codegen::Function;
use codegen::Scope;
use generate_futures::generate_futures;
use std::collections::BTreeMap;

pub fn generate_service_calls(
    scope: &mut Scope,
    function: &mut Function,
    generation_infos: (
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
    workflow_response_code_generation_info: WorkflowResponseCodeGenerationInfo,
    variable_aliases: VariableAliases,
) {
    generate_imports(scope);

    generate_futures(function, &generation_infos, variable_aliases);

    generate_streams(function, generation_infos.1.to_vec());

    generate_response_handling(function, workflow_response_code_generation_info);
}

fn generate_response_handling(
    mut function: &mut Function,
    workflow_response_code_generation_info: WorkflowResponseCodeGenerationInfo,
) {
    function.line("tokio::pin!(merged);");

    for dependency_info in &workflow_response_code_generation_info.dependency_infos {
        function.line(format!(
            "let mut {} = None;",
            dependency_info.result_destructure_variable_name
        ));
    }

    function
        .line("while let Some(result) = merged.next().await {")
        .line("match result {");

    for dependency_info in &workflow_response_code_generation_info.dependency_infos {
        let ServiceCodeGenerationInfo { enum_name, .. } =
            &dependency_info.service_operation_dependency;

        function.line(format!(
            "Message::{}(result) => {} = Some(result.unwrap()),",
            enum_name, dependency_info.result_destructure_variable_name,
        ));
    }

    function.line("_ => {}").line("}").line("}");

    for dependency_info in &workflow_response_code_generation_info.dependency_infos {
        let ServiceCodeGenerationInfo {
            response_struct_name,
            response_aliases,
            ..
        } = &dependency_info.service_operation_dependency;

        function.line(format!("let {} {{", response_struct_name,));

        traverse_nested_node(
            response_aliases.clone(),
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
            &mut (&mut function, response_struct_name),
        );

        function.line(format!(
            "}} = {}.unwrap();",
            dependency_info.result_destructure_variable_name
        ));
    }
}

fn generate_streams(
    function: &mut Function,
    generation_infos_with_ids: Vec<((String, String), ServiceCodeGenerationInfo)>,
) {
    for (_, generation_info) in &generation_infos_with_ids {
        function.line(format!(
            "let {} = futures::FutureExt::into_stream({}.clone()).map(Message::{});",
            generation_info.stream_variable_name,
            generation_info.future_variable_name,
            generation_info.enum_name
        ));
    }

    let mut iter = generation_infos_with_ids
        .iter()
        .map(|(_, info)| info.stream_variable_name.to_string());

    let first_stream_variable = iter.next().unwrap();
    let formatted_merged_streams = iter
        .map(|stream_variable_name| format!(".merge({})", stream_variable_name))
        .collect::<Vec<String>>()
        .join("");

    let all_formatted_merged_streams =
        format!("{}{}", first_stream_variable, formatted_merged_streams);

    function.line(format!("let merged = {};", all_formatted_merged_streams));
}

fn generate_imports(scope: &mut Scope) {
    scope.import("reqwest", "Client");
    scope.import("tokio_stream", "StreamExt");
    scope.import("futures", "FutureExt");
    scope.import("http", "StatusCode");
}
