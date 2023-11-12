use codegen::Function;

use crate::{
    generate_workflows::{
        add_variable_aliases_to_request_parameters::{WorkflowPathPart, WorkflowVariable},
        input_map::Variable,
    },
    parse_specs::parse_schema::to_string_schema,
};

pub fn create_function_signature(
    function: &mut Function,
    path_parts: Vec<WorkflowPathPart>,
    query_struct_name: &str,
) {
    function.vis("pub");
    function.set_async(true);

    create_function_arguments(path_parts, function, query_struct_name);

    function.ret("Result<impl warp::Reply, warp::Rejection>");
}

fn create_function_arguments(
    path_parts: Vec<WorkflowPathPart>,
    function: &mut Function,
    query_struct_name: &str,
) {
    let path_parameters_info: Vec<(String, String)> = path_parts
        .iter()
        .filter(|path_part| (*path_part).schema_type.is_some())
        .map(|path_part| -> (String, String) {
            let name = match path_part.name.clone() {
                WorkflowVariable::Variable(name) => name,
                _ => Variable {
                    original_name: String::new(),
                    alias: String::new(),
                },
            };

            (
                name.alias,
                to_string_schema(path_part.schema_type.unwrap(), None),
            )
        })
        .collect();

    for (name, schema_type) in path_parameters_info {
        function.arg(&name, schema_type);
    }

    function.arg("parameters", query_struct_name);
}
