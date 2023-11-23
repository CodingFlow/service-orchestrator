use crate::generate_workflows::generate_workflow::build_workflow_request_view_data::WorkflowPathPart;
use codegen::Function;

pub fn generate_define_paths(function: &mut Function, path_parts: Vec<WorkflowPathPart>) {
    function.line("let with_paths = http_method");

    for path_part in path_parts {
        let formatted_path_part = match path_part.formatted_type {
            Some(formatted_type) => format!(".and(warp::path::param::<{}>())", formatted_type),
            None => format!(".and(warp::path(\"{}\"))", path_part.name),
        };

        function.line(formatted_path_part);
    }

    function.line(".and(warp::path::end());");
}
