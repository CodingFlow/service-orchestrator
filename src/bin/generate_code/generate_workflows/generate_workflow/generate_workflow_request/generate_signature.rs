use crate::generate_workflows::generate_workflow::build_workflow_request_view_data::WorkflowPathPart;
use codegen::Function;

pub fn generate_signature(
    function: &mut Function,
    path_parameters: Vec<WorkflowPathPart>,
    query_struct_name: &str,
) {
    let mut parameters: Vec<String> = path_parameters
        .iter()
        .filter(|path_part| (*path_part).alias.is_some())
        .map(|path_part| -> String { path_part.formatted_type.clone().unwrap() })
        .collect();

    parameters.push(query_struct_name.to_string());

    let formatted_parameters = parameters.join(",");

    function.vis("pub").ret(format!(
        "impl Filter<Extract = {}, Error = warp::Rejection> + Clone",
        format!("({})", formatted_parameters)
    ));
}
