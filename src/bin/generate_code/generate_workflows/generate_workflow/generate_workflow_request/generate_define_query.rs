use codegen::Scope;

use crate::{
    generate_workflows::{
        add_variable_aliases_to_request_parameters::{WorkflowPathPart, WorkflowVariable},
        input_map::Variable,
    },
    parse_specs::parse_schema::to_string_schema,
};

use super::format_tuple;

pub fn generate_define_query(
    scope: &mut Scope,
    path_parameters: Vec<WorkflowPathPart>,
    query_struct_name: &str,
) {
    let mut all_parameters_return_value: Vec<String> = path_parameters
        .iter()
        .filter(|path_part| (*path_part).schema_type.is_some())
        .map(|path_part| -> String {
            let name = match path_part.name.clone() {
                WorkflowVariable::Variable(name) => name,
                _ => Variable {
                    original_name: String::new(),
                    alias: String::new(),
                },
            };

            to_string_schema(
                path_part.schema_type.unwrap(),
                Some(name.original_name.to_string()),
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
