use codegen::Scope;
use oas3::spec::SchemaType;

use crate::spec_parsing::to_string_schema_type_primitive;

use super::format_tuple;

pub fn generate_define_query(
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
) {
    let mut all_parameters_return_value: Vec<&str> = path_parameters
        .iter()
        .map(|(_, schema_type)| -> &str { to_string_schema_type_primitive(*schema_type) })
        .collect();

    let function = scope.new_fn("define_query").arg(
        "with_paths",
        format!(
            "impl Filter<Extract = {}, Error = Rejection> + Copy",
            format_tuple(all_parameters_return_value.to_vec())
        ),
    );

    all_parameters_return_value.append(&mut vec![query_struct_name]);

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
