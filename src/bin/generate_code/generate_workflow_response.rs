use std::collections::BTreeMap;
use std::fs;

use codegen::{Field, Function, Scope, Struct};
use oas3::Spec;

use oas3::spec::{ObjectOrReference, Response, SchemaType};

use crate::spec_parsing::{parse_schema, to_string_schema_type_primitive, ParsedSchema};

pub fn generate_workflow_response(
    responses: BTreeMap<String, ObjectOrReference<Response>>,
    spec: &Spec,
    (path_parameters, query_parameters): (Vec<(String, SchemaType)>, Vec<(String, SchemaType)>),
    query_struct_name: &str,
) {
    let mut scope = Scope::new();

    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");
    scope.import("warp::reply", "self");
    scope.import("warp::reply", "Json");
    scope.import("crate::workflow_request_definition", query_struct_name);

    let binding = extract_response_values_from_spec(responses, spec);
    let response_values: Vec<(&String, ParsedSchema)> = binding
        .iter()
        .map(|(status_code, response_value)| -> (&String, ParsedSchema) {
            let btree_map = &mut response_value.content.clone();
            let first_entry = btree_map.first_entry();
            let occupied_entry = &first_entry.unwrap();
            let json_response = occupied_entry.get();
            let parsed_schemas = parse_schema(
                vec![(
                    Some(status_code.to_string()),
                    json_response.schema.clone().unwrap().resolve(spec).unwrap(),
                )],
                spec,
            );

            (status_code, parsed_schemas.first().unwrap().clone())
        })
        .collect();

    let status_code_struct_name_pairs =
        generate_response_structure(response_values.to_vec(), &mut scope);
    generate_map_response(
        status_code_struct_name_pairs,
        &mut scope,
        path_parameters,
        query_parameters,
        query_struct_name,
        response_values,
    );

    println!("{}", scope.to_string());

    write_file(scope.to_string());
}

fn generate_map_response(
    status_code_struct_name_pairs: Vec<(String, String)>,
    scope: &mut Scope,
    path_parameters: Vec<(String, SchemaType)>,
    query_parameters: Vec<(String, SchemaType)>,
    query_struct_name: &str,
    response_values: Vec<(&String, ParsedSchema)>,
) {
    let map_functions: Vec<Function> = status_code_struct_name_pairs
        .iter()
        .map(|(status_code, struct_name)| -> Function {
            let mut function = Function::new("map_response");

            function.vis("pub");

            let path_parameters_info: Vec<(&str, &str)> = path_parameters
                .iter()
                .map(|(name, schema_type)| -> (&str, &str) {
                    (name, to_string_schema_type_primitive(*schema_type))
                })
                .collect();

            for (name, schema_type) in path_parameters_info {
                function.arg(name, schema_type);
            }

            function.arg("parameters", query_struct_name);

            function.ret("Json");

            function.line(format!("reply::json(&{} {{", struct_name));

            let parsed_schema = &response_values
                .iter()
                .find(|(parsed_schema_status_code, _)| -> bool {
                    status_code == *parsed_schema_status_code
                })
                .unwrap()
                .1;

            for response_property in parsed_schema.properties.clone().unwrap() {
                function.line(format!("{}:{},", response_property.name.unwrap(), "1.0"));
            }

            function.line("})");

            function
        })
        .collect();

    for function in map_functions {
        scope.push_fn(function);
    }
}

fn generate_response_structure(
    response_values: Vec<(&String, ParsedSchema)>,
    scope: &mut Scope,
) -> Vec<(String, String)> {
    let responses: Vec<(String, String, Struct)> = response_values
        .iter()
        .map(|(status_code, parsed_schema)| -> (String, String, Struct) {
            let struct_name = &format!("WorkflowResponse_{}", status_code);
            let mut new_struct = Struct::new(struct_name);

            new_struct.derive("Serialize").derive("Deserialize");

            let fields: Vec<Field> = parsed_schema
                .properties
                .clone()
                .unwrap()
                .iter()
                .map(|property_schema| -> Field {
                    Field::new(
                        &property_schema.name.clone().unwrap(),
                        to_string_schema_type_primitive(property_schema.schema_type), // assumes no nested objects in response
                    )
                })
                .collect();

            for field in fields {
                new_struct.push_field(field);
            }

            (status_code.to_string(), struct_name.to_string(), new_struct)
        })
        .collect();

    let final_result = responses
        .iter()
        .map(|(status_code, struct_name, _)| -> (String, String) {
            (status_code.to_string(), struct_name.to_string())
        })
        .collect();

    for (_, _, structure) in responses {
        scope.push_struct(structure);
    }

    final_result
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
