use codegen::Scope;

use crate::{
    generate_workflows::{
        add_variable_aliases_to_request_parameters::{WorkflowPathPart, WorkflowVariable},
        input_map::Variable,
    },
    parse_specs::parse_schema::to_string_schema,
};

pub fn generate_define_request(
    scope: &mut Scope,
    path_parameters: Vec<WorkflowPathPart>,
    query_struct_name: &str,
) {
    let mut parameters: Vec<String> = path_parameters
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

    parameters.push(query_struct_name.to_string());

    let formatted_parameters = parameters.join(",");

    scope
        .new_fn("define_request")
        .vis("pub")
        .ret(format!(
            "impl Filter<Extract = {}, Error = warp::Rejection> + Clone",
            format!("({})", formatted_parameters)
        ))
        .line("let http_method = define_method();")
        .line("let with_paths = define_paths(http_method);")
        .line("let with_query = define_query(with_paths);")
        .line("with_query");
}
