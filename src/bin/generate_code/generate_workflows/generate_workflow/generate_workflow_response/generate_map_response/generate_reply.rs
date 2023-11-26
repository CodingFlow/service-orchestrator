use codegen::Function;

use crate::generate_workflows::generate_workflow::{
    build_service_call_view_data::generate_body_variables::generate_body_variables,
    build_workflow_response_view_data::WorkflowResponseGenerationInfo,
};

pub fn generate_reply(
    function: &mut Function,
    workflow_response_generation_info: WorkflowResponseGenerationInfo,
) {
    // TODO: handle more than one status code
    let response_alias = &workflow_response_generation_info
        .generation_infos
        .first()
        .unwrap()
        .body;

    function.line("Ok(reply::json(&");

    generate_body_variables(function, &response_alias.clone(), true);

    function.line("))");
}
