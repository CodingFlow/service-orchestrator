mod generate_futures;

use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::traversal::traverse_nested_node;
use crate::traversal::NestedNode;

use super::build_service_operation_lookup_map::ServiceCodeGenerationInfo;
use super::build_service_operation_lookup_map::ServiceResponseAlias;
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
    variable_aliases: &mut VariableAliases,
) {
    generate_imports(scope);

    generate_futures(function, &generation_infos, variable_aliases);

    generate_streams(function, generation_infos.1.to_vec());

    generate_response_handling(function, workflow_response_code_generation_info);
}

fn generate_response_handling(
    function: &mut Function,
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
            response_aliases, ..
        } = &dependency_info.service_operation_dependency;

        generate_response_variables(function, response_aliases);

        function.line(format!(
            "= {}.unwrap();",
            dependency_info.result_destructure_variable_name
        ));
    }
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
