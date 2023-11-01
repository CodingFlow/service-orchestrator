use codegen::Function;
use oas3::spec::SchemaType;

pub fn create_query_destructure(
    function: &mut Function,
    query_struct_name: &str,
    query_parameters: &Vec<(String, SchemaType)>,
) {
    function.line(format_query_destructure(
        query_struct_name,
        query_parameters.clone(),
    ));
}

fn format_query_destructure(
    query_struct_name: &str,
    query_parameters: Vec<(String, SchemaType)>,
) -> String {
    let variables: Vec<String> = query_parameters
        .iter()
        .map(|(name, _)| -> String { format!("mut {}", name) })
        .collect();

    format!(
        "let {} {{ {} }} = parameters;",
        query_struct_name,
        variables.join(",")
    )
}
