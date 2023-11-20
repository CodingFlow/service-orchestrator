use codegen::Scope;

use crate::generate_workflows::generate_workflow::build_request_view_data::WorkflowPathPart;

use super::format_tuple;

pub fn generate_define_query(
    scope: &mut Scope,
    path_parameters: Vec<WorkflowPathPart>,
    query_struct_name: &str,
) {
    let mut all_parameters_return_value: Vec<String> = path_parameters
        .iter()
        .filter(|path_part| (*path_part).alias.is_some())
        .map(|path_part| -> String { path_part.formatted_type.clone().unwrap() })
        .collect();

    let function = scope.new_fn("define_query").arg(
        "with_paths",
        format!(
            "impl Filter<Extract = {}, Error = Rejection> + Copy",
            format_tuple(all_parameters_return_value.to_vec())
        ),
    );

    all_parameters_return_value.append(&mut vec![query_struct_name.to_string()]);

    function
        .ret(format!(
            "impl Filter<Extract = {}, Error = Rejection> + Copy",
            format!("({})", all_parameters_return_value.join(","))
        ))
        .line(format!(
            "with_paths.and(warp::query::<{}>())",
            query_struct_name
        ));
}
