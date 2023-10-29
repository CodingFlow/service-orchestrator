use oas3::{
    spec::{Operation, Parameter, PathItem, SchemaType},
    Schema, Spec,
};

pub fn extract_request_values_from_spec<'a>(
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
