use std::fs::{self};

use codegen::{Field, Scope, Struct};
use http::Method;
use oas3::{
    spec::{Operation, Parameter, PathItem, SchemaType},
    Schema, Spec,
};

use crate::spec_parsing::to_string_schema_type_primitive;

pub fn generate_workflow_request(
    path_item: &PathItem,
    operation: &Operation,
    spec: &Spec,
    method: Method,
    path_string: &String,
) {
    let mut scope = Scope::new();

    scope.import("warp::reject", "Rejection");
    scope.import("warp", "Filter");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");

    let (path_parameters, query_parameters) =
        extract_request_values_from_spec(path_item, operation, spec);

    let query_struct_name = generate_query_struct(&mut scope, query_parameters);

    generate_define_request(&mut scope, path_parameters.to_vec(), query_struct_name);
    generate_define_method(&mut scope, method);
    generate_define_paths(&mut scope, path_string, path_parameters.to_vec());
    generate_define_query(&mut scope, path_parameters, query_struct_name);

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn generate_query_struct(
    scope: &mut Scope,
    query_parameters: Vec<(String, SchemaType)>,
) -> &'static str {
    let fields = query_parameters.iter().map(|(name, schema_type)| -> Field {
        Field::new(name, to_string_schema_type_primitive(*schema_type))
    });

    let mut new_struct = Struct::new("QueryParameters");

    new_struct.derive("Serialize").derive("Deserialize");

    for field in fields {
        new_struct.push_field(field);
    }

    scope.push_struct(new_struct);

    "QueryParameters"
}

fn extract_request_values_from_spec<'a>(
    path_item: &'a PathItem,
    operation: &'a Operation,
    spec: &'a Spec,
) -> (Vec<(String, SchemaType)>, Vec<(String, SchemaType)>) {
    let (path_parameters, query_parameters) = extract_parameters(path_item, operation, spec);

    (path_parameters, query_parameters)
}

fn extract_parameters(
    path_item: &PathItem,
    operation: &Operation,
    spec: &Spec,
) -> (Vec<(String, SchemaType)>, Vec<(String, SchemaType)>) {
    let mut all_parameters = path_item.parameters.to_vec();

    all_parameters.extend(operation.parameters.to_vec());

    let all_resolved_parameters = all_parameters
        .iter()
        .map(|reference| -> Parameter { reference.resolve(&spec).unwrap() });

    let path_parameters: Vec<(String, SchemaType)> = all_resolved_parameters
        .clone()
        .filter(|parameter| -> bool { parameter.location == "path" })
        .map(|parameter| -> Parameter { parameter.clone() })
        .map(|parameter| -> (String, Schema) { (parameter.name, parameter.schema.unwrap()) })
        .map(|(name, schema)| -> (String, oas3::spec::SchemaType) {
            (name, schema.schema_type.unwrap())
        })
        .collect();

    let query_parameters: Vec<(String, SchemaType)> = all_resolved_parameters
        .filter(|parameter| -> bool { parameter.location == "query" })
        .map(|parameter| -> Parameter { parameter.clone() })
        .map(|parameter| -> (String, Schema) { (parameter.name, parameter.schema.unwrap()) })
        .map(|(name, schema)| -> (String, oas3::spec::SchemaType) {
            (name, schema.schema_type.unwrap())
        })
        .collect();
    (path_parameters, query_parameters)
}

fn generate_define_request(
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
) {
    let mut parameters = path_parameters
        .iter()
        .map(|(_, schema_type)| -> &str { to_string_schema_type_primitive(*schema_type) })
        .collect::<Vec<&str>>();

    parameters.push(query_struct_name);

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

fn generate_define_method(scope: &mut Scope, method: Method) {
    let function = scope
        .new_fn("define_method")
        .ret("impl Filter<Extract = (), Error = Rejection> + Copy");

    let method = method.as_str().to_lowercase();
    function.line(format!("warp::{}()", method));
}

fn generate_define_paths(
    scope: &mut Scope,
    path_string: &String,
    path_parameters: Vec<(String, SchemaType)>,
) {
    let formatted_parameters: Vec<&str> = path_parameters
        .iter()
        .map(|(_, schema_type)| -> &str { to_string_schema_type_primitive(*schema_type) })
        .collect();

    let function = scope
        .new_fn("define_paths")
        .arg(
            "http_method",
            "impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy",
        )
        .ret(format!(
            "impl Filter<Extract = {}, Error = warp::reject::Rejection> + Copy",
            format_tuple(formatted_parameters)
        ))
        .line("http_method");

    let path_parts = path_string.split('/');

    for path_part in path_parts {
        match (path_part.get(..1), path_part.chars().rev().nth(0)) {
            (Some("{"), Some('}')) => function.line(format!(
                ".and(warp::path::param::<{}>())",
                to_string_schema_type_primitive(
                    path_parameters
                        .iter()
                        .find(|(name, _)| -> bool {
                            name.as_str() == remove_first_and_last(path_part)
                        })
                        .unwrap()
                        .1
                )
            )),
            (Some(_), Some(_)) => function.line(format!(".and(warp::path(\"{}\"))", path_part)),
            _ => function,
        };
    }

    function.line(".and(warp::path::end())");
}

fn format_tuple(input: Vec<&str>) -> String {
    match input.len() {
        1 => format!("({},)", input.join(",")),
        _ => format!("({})", input.join(",")),
    }
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn generate_define_query(
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

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_request_definition.rs", code);
}
