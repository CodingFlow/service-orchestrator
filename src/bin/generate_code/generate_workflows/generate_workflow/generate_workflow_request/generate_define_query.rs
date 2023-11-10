use codegen::Scope;

use crate::{
    generate_workflows::extract_request_parameters_from_spec::RequestParameter,
    spec_parsing::to_string_schema,
};

use super::format_tuple;

pub fn generate_define_query(
    scope: &mut Scope,
    path_parameters: Vec<RequestParameter>,
    query_struct_name: &str,
) {
    let mut all_parameters_return_value: Vec<String> = path_parameters
        .iter()
        .map(|parameter| -> String {
            to_string_schema(
                parameter.schema_type,
                Some(parameter.name.original_name.to_string()),
            )
        })
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
