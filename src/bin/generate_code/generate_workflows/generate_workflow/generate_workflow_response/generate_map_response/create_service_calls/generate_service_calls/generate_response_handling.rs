use super::super::build_service_operation_lookup_map::ServiceCodeGenerationInfo;
use super::super::build_workflow_response_lookup_map::WorkflowResponseCodeGenerationInfo;
use super::generate_response_variables_assigned;
use codegen::Function;

pub fn generate_response_handling(
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

        generate_response_variables_assigned(function, response_aliases);

        function.line(format!(
            "= {}.unwrap();",
            dependency_info.result_destructure_variable_name
        ));
    }
}
