mod generate_workflow_request;
mod generate_workflow_response;

use http::Method;
use oas3::{
    spec::{Operation, PathItem},
    Spec,
};

use generate_workflow_request::generate_workflow_request;
use generate_workflow_response::generate_workflow_response;

use crate::{
    extract_request_values_from_spec::extract_request_values_from_spec,
    generate_re_exports::ReExports,
};

pub fn generate_workflow(
    path_item: &PathItem,
    operation: &Operation,
    spec: &Spec,
    method: Method,
    path_string: &String,
    re_exports: &mut ReExports,
) {
    let request_values_from_spec = extract_request_values_from_spec(path_item, operation, spec);

    let query_struct_name = generate_workflow_request(
        method,
        path_string.to_string(),
        request_values_from_spec.clone(),
        re_exports,
    );
    generate_workflow_response(
        operation.responses.clone(),
        spec,
        request_values_from_spec,
        query_struct_name,
        re_exports,
    );
}
