use oas3::{
    spec::{Operation, Parameter, PathItem, SchemaType},
    Schema, Spec,
};

use super::input_map::{InputMap, InputMapBehavior, Variable};

#[derive(Debug, Clone)]
pub struct RequestParameters {
    pub path_parameters: Vec<RequestParameter>,
    pub query_parameters: Vec<RequestParameter>,
}

#[derive(Debug, Clone)]
pub struct RequestParameter {
    pub name: Variable,
    pub schema_type: SchemaType,
}

pub fn extract_request_parameters_from_spec<'a>(
    path_item: &'a PathItem,
    operation: &'a Operation,
    spec: &'a Spec,
    input_map: &mut InputMap,
) -> RequestParameters {
    let (path_parameters, query_parameters) =
        extract_parameters(path_item, operation, spec, input_map);

    RequestParameters {
        path_parameters,
        query_parameters,
    }
}

fn extract_parameters(
    path_item: &PathItem,
    operation: &Operation,
    spec: &Spec,
    input_map: &mut InputMap,
) -> (Vec<RequestParameter>, Vec<RequestParameter>) {
    let mut all_parameters = path_item.parameters.to_vec();

    all_parameters.extend(operation.parameters.to_vec());

    let all_resolved_parameters: Vec<Parameter> = all_parameters
        .iter()
        .map(|reference| -> Parameter { reference.resolve(&spec).unwrap() })
        .collect();

    let path_parameters = filter_map_parameters(
        input_map,
        all_resolved_parameters.to_vec(),
        |parameter| -> bool { parameter.location == "path" },
    );

    let query_parameters =
        filter_map_parameters(input_map, all_resolved_parameters, |parameter| -> bool {
            parameter.location == "query"
        });

    (path_parameters, query_parameters)
}

fn filter_map_parameters(
    input_map: &mut InputMap,
    all_resolved_parameters: Vec<Parameter>,
    filter_function: fn(&Parameter) -> bool,
) -> Vec<RequestParameter> {
    let path_parameters: Vec<RequestParameter> = all_resolved_parameters
        .to_vec()
        .into_iter()
        .filter(filter_function)
        .map(|parameter| parameter.clone())
        .map(|parameter| -> (String, Schema) { (parameter.name, parameter.schema.unwrap()) })
        .map(|(name, schema)| -> RequestParameter {
            RequestParameter {
                name: input_map.create_variable_alias(name.to_string()),
                schema_type: schema.schema_type.unwrap(),
            }
        })
        .collect();
    path_parameters
}
