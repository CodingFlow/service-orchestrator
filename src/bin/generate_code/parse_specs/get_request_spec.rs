use super::parse_schema::parse_schema;
use super::PathPart;
use super::RequestSpec;
use super::SpecInfo;
use http::Method;
use oas3::spec::Operation;
use oas3::spec::Parameter;
use oas3::spec::PathItem;
use oas3::spec::SchemaType;
use std::collections::BTreeMap;

pub fn get_request_specs(
    spec_infos: Vec<SpecInfo>,
) -> (
    BTreeMap<(String, String), RequestSpec>,
    Vec<(SpecInfo, Operation)>,
) {
    let tuples: Vec<(((String, String), RequestSpec), (SpecInfo, Operation))> = spec_infos
        .iter()
        .flat_map(get_path_data)
        .flat_map(get_operation_data)
        .map(get_operation_request_parameters)
        .map(get_operation_path_parts)
        .map(get_operation_query_parameters)
        .map(create_request_spec_with_id)
        .collect();

    let (request_tuples, left_over_tuples): (
        Vec<((String, String), RequestSpec)>,
        Vec<(SpecInfo, Operation)>,
    ) = tuples.into_iter().unzip();

    let request_map: BTreeMap<(String, String), RequestSpec> = request_tuples.into_iter().collect();

    (request_map, left_over_tuples)
}

fn get_path_data(spec_info: &SpecInfo) -> Vec<(&SpecInfo, String, &PathItem)> {
    spec_info
        .spec
        .paths
        .iter()
        .map(
            |(path_string, path_item)| -> (&SpecInfo, String, &PathItem) {
                (spec_info, path_string.to_string(), path_item)
            },
        )
        .collect()
}

fn get_operation_data(
    (spec_info, path_string, path_item): (&SpecInfo, String, &PathItem),
) -> Vec<(SpecInfo, String, PathItem, Method, Operation)> {
    path_item
        .methods()
        .into_iter()
        .map(
            |(method, operation)| -> (SpecInfo, String, PathItem, Method, Operation) {
                (
                    spec_info.clone(),
                    path_string.to_string(),
                    path_item.clone(),
                    method,
                    operation.clone(),
                )
            },
        )
        .collect()
}

fn get_operation_request_parameters(
    (spec_info, path_string, path_item, method, operation): (
        SpecInfo,
        String,
        PathItem,
        Method,
        Operation,
    ),
) -> (
    SpecInfo,
    std::string::String,
    Vec<oas3::spec::Parameter>,
    Method,
    Operation,
) {
    let mut operation_parameters: Vec<Parameter> = operation
        .parameters
        .iter()
        .map(|object_ref| object_ref.resolve(&spec_info.spec).unwrap())
        .collect();

    let parameters_to_add: Vec<Parameter> = path_item
        .parameters
        .iter()
        .map(|object_ref| object_ref.resolve(&spec_info.spec).unwrap())
        .filter(|parameter| {
            !operation_parameters
                .iter()
                .any(|operation_parameter| operation_parameter.name == parameter.name)
        })
        .collect();

    operation_parameters.extend(parameters_to_add);

    (
        spec_info,
        path_string,
        operation_parameters,
        method,
        operation,
    )
}

fn get_operation_path_parts(
    (spec_info, path_string, parameters, method, operation): (
        SpecInfo,
        std::string::String,
        Vec<oas3::spec::Parameter>,
        Method,
        Operation,
    ),
) -> (SpecInfo, Vec<PathPart>, Vec<Parameter>, Method, Operation) {
    let path_parts = path_string
        .split("/")
        .skip(1)
        .map(|path_part| {
            let is_parameter = path_part.chars().next().unwrap() == '{';

            let path_part = match is_parameter {
                true => remove_first_and_last(path_part),
                false => path_part,
            };

            let parameter_info = match is_parameter {
                true => Some(
                    parameters
                        .clone()
                        .into_iter()
                        .find(|parameter| parameter.name == path_part)
                        .unwrap(),
                ),
                false => None,
            };

            PathPart {
                name: path_part.to_string(),
                parameter_info,
            }
        })
        .collect();

    (spec_info, path_parts, parameters, method, operation)
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn get_operation_query_parameters(
    (spec_info, path_parts, parameters, method, operation): (
        SpecInfo,
        Vec<PathPart>,
        Vec<Parameter>,
        Method,
        Operation,
    ),
) -> (
    SpecInfo,
    Vec<PathPart>,
    BTreeMap<String, SchemaType>,
    Method,
    Operation,
) {
    let query_parameters = parameters
        .iter()
        .filter(|parameter| (*parameter).location == "query")
        .map(|parameter| {
            (
                parameter.name.clone(),
                parameter.schema.clone().unwrap().schema_type.unwrap(),
            )
        })
        .collect();

    (spec_info, path_parts, query_parameters, method, operation)
}

fn create_request_spec_with_id(
    (spec_info, path_parts, query_parameters, method, operation): (
        SpecInfo,
        Vec<PathPart>,
        BTreeMap<String, SchemaType>,
        Method,
        Operation,
    ),
) -> (
    ((std::string::String, std::string::String), RequestSpec),
    (SpecInfo, Operation),
) {
    let body = match operation.request_body.clone() {
        Some(request_body_ref) => {
            let request_body = request_body_ref.resolve(&spec_info.spec).unwrap();
            let media_type = request_body.content.get("application/json").unwrap();
            let body_schema = media_type.schema(&spec_info.spec).unwrap();
            Some(parse_schema(body_schema, &spec_info.spec))
        }
        None => None,
    };

    (
        (
            ((
                spec_info.clone().name,
                operation.operation_id.clone().unwrap(),
            )),
            RequestSpec {
                method,
                query: query_parameters,
                path: path_parts,
                body,
            },
        ),
        (spec_info, operation),
    )
}
