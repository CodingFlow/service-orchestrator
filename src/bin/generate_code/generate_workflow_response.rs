use std::collections::BTreeMap;
use std::fs;

use codegen::{Field, Scope, Struct};
use http::StatusCode;
use oas3::{Spec, Schema};

use oas3::spec::{ObjectOrReference, Response};

use crate::spec_parsing::parse_schema;

pub fn generate_workflow_response(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
) {
    let mut scope = Scope::new();

    let response_values: BTreeMap<String, Response> =
        extract_response_values_from_spec(responses, spec);

    generate_response_structure(response_values, &mut scope, &spec);

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn generate_response_structure(response_values: BTreeMap<String, Response>, scope: &mut Scope, spec: &Spec) {
    let response = scope.new_struct("WorkflowResponse");

    let responses: Vec<Struct> = response_values
        .iter()
        .map(|(status_code, response_value)| -> (&String, BTreeMap<String, ObjectOrReference<Schema>>) { 
            let json_response = response_value.content.first_entry().unwrap().get();
            let properties = parse_schema(vec![(None, json_response.schema.unwrap().resolve(spec).unwrap())], spec);

            (status_code, properties)
         })
         .map(|(status_code, wrapped_properties)| -> (String, Struct){
            let fields: Vec<Field> = wrapped_properties
            .iter()
            .map(|reference_property| -> Field {
                let property_wrapped = reference_property.1.resolve(spec).unwrap().items.unwrap();
                let property = property_wrapped.resolve(spec).unwrap();

                Field::new(property., )
            })
            .collect();

            let response = Struct::new("WorkflowResponse");

            for field in fields {
                response.field(field., ty)
            }

         })
        .collect();
}

fn extract_response_values_from_spec(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
) -> BTreeMap<String, Response> {
    let a = responses.clone();

    a.iter()
        .map(|(status_code, wrapped_response)| -> (String, Response) {
            (
                status_code.to_string(),
                wrapped_response.resolve(spec).unwrap(),
            )
        })
        .collect()
}

fn write_file(code: String) {
    let _ = fs::write("./src/workflow_response_definition.rs", code);
}
