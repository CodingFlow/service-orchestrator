use std::fs::{self};

use codegen::Scope;
use http::Method;
use oas3::{
    spec::{Operation, Parameter, PathItem, SchemaType},
    Schema, Spec,
};

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
    scope.import("std::collections", "HashMap");

    let (parameter_types_with_name, parameter_types) =
        extract_request_values_from_spec(path_item, operation, spec);

    generate_define_request(&mut scope, parameter_types.to_vec());
    generate_define_method(&mut scope, method);
    generate_define_paths(
        &mut scope,
        path_string,
        parameter_types_with_name.to_vec(),
        parameter_types.to_vec(),
    );
    generate_define_query(&mut scope, path_item, parameter_types.to_vec());

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn extract_request_values_from_spec<'a>(
    path_item: &'a PathItem,
    operation: &'a Operation,
    spec: &'a Spec,
) -> (Vec<(String, &'a str)>, Vec<&'a str>) {
    let mut all_parameters = path_item.parameters.to_vec();

    all_parameters.extend(operation.parameters.to_vec());

    let parameter_types_with_name: Vec<(String, &str)> = all_parameters
        .iter()
        .map(|reference| -> Parameter { reference.resolve(&spec).unwrap() })
        .filter(|parameter| -> bool { parameter.location == "path" })
        .map(|parameter| -> Parameter { parameter.clone() })
        .map(|parameter| -> (String, Schema) { (parameter.name, parameter.schema.unwrap()) })
        .map(|(name, schema)| -> (String, oas3::spec::SchemaType) {
            (name, schema.schema_type.unwrap())
        })
        .map(|(name, schema_type)| -> (String, &str) {
            let r#type = match schema_type {
                SchemaType::Boolean => "bool",
                SchemaType::Integer => "i32",
                SchemaType::Number => "f32",
                SchemaType::String => "String",
                SchemaType::Array => "array",
                SchemaType::Object => "todo!()",
            };

            (name, r#type)
        })
        .collect();

    let parameter_types: Vec<&str> = parameter_types_with_name
        .to_vec()
        .iter()
        .map(|(_, schema_type)| -> &str { schema_type })
        .collect();
    (parameter_types_with_name, parameter_types)
}

fn generate_define_request(scope: &mut Scope, mut parameter_types: Vec<&str>) {
    parameter_types.extend(vec!["HashMap<String, String>"]);

    scope
        .new_fn("define_request")
        .vis("pub")
        .ret(format!(
            "impl Filter<Extract = {}, Error = warp::Rejection> + Clone",
            format!("({})", parameter_types.join(","))
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
    parameter_types_with_name: Vec<(String, &str)>,
    parameter_types: Vec<&str>,
) {
    let function = scope
        .new_fn("define_paths")
        .arg(
            "http_method",
            "impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy",
        )
        .ret(format!(
            "impl Filter<Extract = {}, Error = warp::reject::Rejection> + Copy",
            format!("({},)", parameter_types.join(","))
        ))
        .line("http_method");

    let path_parts = path_string.split('/');

    for path_part in path_parts {
        match (path_part.get(..1), path_part.chars().rev().nth(0)) {
            (Some("{"), Some('}')) => function.line(format!(
                ".and(warp::path::param::<{}>())",
                parameter_types_with_name
                    .iter()
                    .find(|(name, _)| -> bool { name.as_str() == remove_first_and_last(path_part) })
                    .unwrap()
                    .1
            )),
            (Some(_), Some(_)) => function.line(format!(".and(warp::path(\"{}\"))", path_part)),
            _ => function,
        };
    }

    function.line(".and(warp::path::end())");
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn generate_define_query(scope: &mut Scope, path_item: &PathItem, mut parameter_types: Vec<&str>) {
    let function = scope.new_fn("define_query").arg(
        "with_paths",
        format!(
            "impl Filter<Extract = {}, Error = Rejection> + Copy",
            format!("({},)", parameter_types.join(","))
        ),
    );

    parameter_types.extend(vec!["HashMap<String, String>"]);

    function
        .ret(format!(
            "impl Filter<Extract = {}, Error = Rejection> + Copy",
            format!("({})", parameter_types.join(","))
        ))
        .line("with_paths.and(warp::query::<HashMap<String, String>>())");
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_request_definition.rs", code);
}
