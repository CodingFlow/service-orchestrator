use crate::generate_workflows::generate_workflow::build_workflow_request_view_data::{
    QueryVariables, RequestParameter,
};
use codegen::Function;
use oas3::spec::SchemaType;

pub fn generate_query_destructure(
    function: &mut Function,
    query_variables: QueryVariables,
    query_parameters: Vec<RequestParameter>,
) {
    if query_parameters.len() > 0 {
        function.line(format_query_destructure(
            query_variables,
            query_parameters.to_vec(),
        ));

        for parameter in query_parameters {
            function.line(format_default_values(parameter));
        }
    }
}

fn format_default_values(parameter: RequestParameter) -> String {
    match parameter.schema_type {
        SchemaType::String => format!(
            "let {} = {}.get_or_insert({}).to_string();",
            parameter.name.alias,
            parameter.name.alias,
            convert_type_to_default_value(parameter.schema_type)
        ),
        _ => format!(
            "let {} = *{}.get_or_insert({});",
            parameter.name.alias,
            parameter.name.alias,
            convert_type_to_default_value(parameter.schema_type)
        ),
    }
}

fn format_query_destructure(
    query_variables: QueryVariables,
    query_parameters: Vec<RequestParameter>,
) -> String {
    let variables: Vec<String> = query_parameters
        .iter()
        .map(|parameter| -> String {
            format!(
                "{}: mut {}",
                parameter.name.original_name, parameter.name.alias
            )
        })
        .collect();

    format!(
        "let {} {{ {} }} = {};",
        query_variables.struct_name,
        variables.join(","),
        query_variables.local_variable
    )
}

fn convert_type_to_default_value(schema_type: SchemaType) -> String {
    match schema_type {
        SchemaType::Boolean => "false".to_string(),
        SchemaType::Integer => "0".to_string(),
        SchemaType::Number => "0.0".to_string(),
        SchemaType::String => "String::new()".to_string(),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
    }
}
