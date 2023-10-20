use std::fs::{self};

use codegen::{Block, Function, Scope};
use http::Method;
use oas3::spec::PathItem;

fn main() {
    let spec = parse_config();

    for path in spec.paths {
        for method in path.1.methods() {
            generate_code(&path.1, method.0);
        }
    }
}

fn generate_code(path_item: &PathItem, method: Method) {
    let mut scope = Scope::new();

    scope.import("oas3", "Spec");
    scope.import("warp::reject", "Rejection");
    scope.import("warp", "Filter");

    let mut path_item_match = Block::new("let path_item = match spec.paths.first_key_value()");

    path_item_match
        .line("Some(item) => item.1,")
        .line("None => panic!(\"Endpoint method missing\")")
        .after(";");

    let function = scope
        .new_fn("define_method")
        .arg("spec", "Spec")
        .ret("impl Filter<Extract = (), Error = Rejection> + Copy")
        .push_block(path_item_match);

    add_http_method(method, function);

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn add_http_method(method: Method, function: &mut Function) {
    let method = method.as_str();
    function.line(format!("warp::{}()", method));
}

fn write_file(code: String) {
    let _ = fs::write("./src/test.rs", code);
}

fn parse_config() -> oas3::Spec {
    let spec = match oas3::from_path("./src/workflow_spec.yaml") {
        Ok(spec) => spec,
        Err(_) => panic!("unable to read open API spec file"),
    };

    spec
}
