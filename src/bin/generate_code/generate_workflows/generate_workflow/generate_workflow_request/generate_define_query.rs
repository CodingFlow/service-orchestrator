use crate::generate_workflows::generate_workflow::build_workflow_request_view_data::RequestParameter;
use codegen::Function;

pub fn generate_define_query(
    function: &mut Function,
    query: Vec<RequestParameter>,
    query_struct_name: &str,
) {
    if query.len() > 0 {
        function.line(format!(".and(warp::query::<{}>())", query_struct_name));
    }
}
