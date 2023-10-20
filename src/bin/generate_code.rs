use std::fs::{self};

use codegen::Scope;
use http::Method;
use oas3::spec::PathItem;

fn main() {
    let spec = parse_config();

    for path in spec.paths {
        for method in path.1.methods() {
            generate_code(&path.0, &path.1, method.0);
        }
    }
}

fn generate_code(path_string: &String, path_item: &PathItem, method: Method) {
    let mut scope = Scope::new();

    scope.import("warp::reject", "Rejection");
    scope.import("warp", "Filter");
    scope.import("std::collections", "HashMap");

    generate_define_request(&mut scope);
    generate_define_method(&mut scope, method);
    generate_define_paths(&mut scope, path_string);
    generate_define_query(&mut scope, path_item);

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn generate_define_request(scope: &mut Scope) {
    scope.new_fn("define_request")
    .vis("pub")
    .ret("impl Filter<Extract = (String, HashMap<String, String>), Error = warp::Rejection> + Clone")
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

fn generate_define_paths(scope: &mut Scope, path_string: &String) {
    let function = scope
        .new_fn("define_paths")
        .arg(
            "http_method",
            "impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy",
        )
        .ret("impl Filter<Extract = (String,), Error = warp::reject::Rejection> + Copy")
        .line("http_method");

    let path_parts = path_string.split('/');

    for path_part in path_parts {
        match (path_part.get(..1), path_part.chars().rev().nth(0)) {
            (Some("{"), Some('}')) => function.line(".and(warp::path::param::<String>())"),
            (Some(_), Some(_)) => function.line(format!(".and(warp::path(\"{}\"))", path_part)),
            _ => function,
        };
    }

    function.line(".and(warp::path::end())");
}

fn generate_define_query(scope: &mut Scope, path_item: &PathItem) {
    scope
        .new_fn("define_query")
        .arg(
            "with_paths",
            "impl Filter<Extract = (String,), Error = Rejection> + Copy",
        )
        .ret("impl Filter<Extract = (String, HashMap<String, String>), Error = Rejection> + Copy")
        .line("with_paths.and(warp::query::<HashMap<String, String>>())");
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_request_definition.rs", code);
}

fn parse_config() -> oas3::Spec {
    let spec = match oas3::from_path("./src/workflow_spec.yaml") {
        Ok(spec) => spec,
        Err(_) => panic!("unable to read open API spec file"),
    };

    spec
}
