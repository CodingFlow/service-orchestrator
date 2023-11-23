use codegen::Function;

pub fn generate_define_query(function: &mut Function, query_struct_name: &str) {
    function.line(format!(
        "with_paths.and(warp::query::<{}>())",
        query_struct_name
    ));
}
