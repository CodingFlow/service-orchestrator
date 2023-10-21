mod generate_workflow_request;

use generate_workflow_request::generate_workflow_request;
use http::Method;
use oas3::{
    spec::{Operation, PathItem},
    Spec,
};

fn main() {
    let spec = parse_config();

    for path in &spec.paths {
        for method in path.1.methods() {
            generate_code(&path.0, &path.1, method, &spec);
        }
    }
}

fn parse_config() -> oas3::Spec {
    let spec = match oas3::from_path("./src/workflow_spec.yaml") {
        Ok(spec) => spec,
        Err(_) => panic!("unable to read open API spec file"),
    };

    spec
}

fn generate_code(
    path_string: &String,
    path_item: &PathItem,
    (method, operation): (Method, &Operation),
    spec: &Spec,
) {
    generate_workflow_request(path_item, operation, spec, method, path_string);
}
