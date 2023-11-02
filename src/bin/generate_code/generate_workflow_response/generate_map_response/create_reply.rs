use codegen::Function;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::{spec_parsing::ParsedSchema, traversal::NestedNode};

pub fn create_reply(
    function: &mut Function,
    status_code_struct_name_node: NestedNode<(String, String)>,
    parsed_spec_responses: Vec<(String, ParsedSchema)>,
    input_map: Map<String, Value>,
    query_parameters: Vec<(String, SchemaType)>,
) {
    let (_, struct_name) = &status_code_struct_name_node.current;

    function.line(format!("reply::json(&{} {{", struct_name));

    create_properties(
        parsed_spec_responses,
        status_code_struct_name_node,
        function,
        input_map,
        query_parameters,
    );

    function.line("})");
}

fn create_properties(
    parsed_spec_responses: Vec<(String, ParsedSchema)>,
    status_code_struct_name_node: NestedNode<(String, String)>,
    function: &mut Function,
    input_map: Map<String, Value>,
    query_parameters: Vec<(String, SchemaType)>,
) {
    let (status_code, _) = status_code_struct_name_node.current;
    let parsed_schema =
        find_response_schema_with_matching_status_code(parsed_spec_responses, status_code);

    for (index, response_property) in parsed_schema.properties.clone().unwrap().iter().enumerate() {
        let status_code_struct_name_child_node = status_code_struct_name_node
            .children
            .clone()
            .unwrap()
            .get(index)
            .unwrap()
            .clone();
        create_response_field(
            function,
            status_code_struct_name_child_node,
            response_property.clone(),
            &input_map,
            &query_parameters,
        );
    }
}

fn find_response_schema_with_matching_status_code(
    parsed_spec_responses: Vec<(String, ParsedSchema)>,
    status_code: String,
) -> ParsedSchema {
    let parsed_schema = &parsed_spec_responses
        .iter()
        .find(|(parsed_schema_status_code, _)| -> bool {
            status_code == parsed_schema_status_code.to_string()
        })
        .unwrap()
        .1;

    parsed_schema.clone()
}

fn create_response_field(
    function: &mut Function,
    status_code_struct_name_node: NestedNode<(String, String)>,
    response_property: ParsedSchema,
    input_map: &Map<String, Value>,
    query_parameters: &Vec<(String, SchemaType)>,
) {
    match response_property.schema_type {
        SchemaType::Array => todo!(),
        SchemaType::Object => create_response_field_object(
            function,
            response_property,
            input_map,
            status_code_struct_name_node,
            query_parameters,
        ),
        _ => create_response_field_primitive(
            function,
            response_property,
            input_map,
            query_parameters,
        ),
    }
}

fn create_response_field_object(
    function: &mut Function,
    response_property: ParsedSchema,
    input_map: &Map<String, Value>,
    status_code_struct_name_node: NestedNode<(String, String)>,
    query_parameters: &Vec<(String, SchemaType)>,
) {
    let property_name = response_property.name.clone().unwrap();
    let mapped_value_map = input_map.get(&property_name).unwrap().as_object().unwrap();

    {
        let property_name = property_name;
        let response_property = response_property;
        let (_, struct_name) = status_code_struct_name_node.current;

        function.line(format!("{}:{} {{", property_name, struct_name));

        if let Some(child_properties) = response_property.properties {
            let child_properties = child_properties.iter().enumerate();

            for (index, child_response_property) in child_properties {
                let child_status_code_struct_name_node = status_code_struct_name_node
                    .children
                    .clone()
                    .unwrap()
                    .get(index)
                    .unwrap()
                    .clone();

                let child_input_map = mapped_value_map
                    .get(&child_response_property.name.clone().unwrap())
                    .unwrap()
                    .as_object()
                    .unwrap();

                create_response_field(
                    function,
                    child_status_code_struct_name_node,
                    child_response_property.clone(),
                    child_input_map,
                    query_parameters,
                );
            }
        }

        function.line("}},");
    };
}

fn create_response_field_primitive(
    function: &mut Function,
    response_property: ParsedSchema,
    input_map: &Map<String, Value>,
    query_parameters: &Vec<(String, SchemaType)>,
) {
    let query_parameters = query_parameters.to_vec();
    let property_name = response_property.name.unwrap();
    let mapped_value_name = input_map.get(&property_name).unwrap().as_str().unwrap();

    function.line(format!(
        "{}:{},",
        property_name,
        format_response_field_value(
            response_property.schema_type,
            query_parameters,
            mapped_value_name
        ),
    ));
}

fn format_response_field_value(
    response_property_schema_type: SchemaType,
    query_parameters: Vec<(String, SchemaType)>,
    mapped_value_name: &str,
) -> String {
    {
        let response_property_schema_type = response_property_schema_type;
        let mapped_value_name = mapped_value_name;
        match is_query_parameter(query_parameters, mapped_value_name) {
            true => format!(
                "*{}.get_or_insert({})",
                mapped_value_name,
                convert_type_to_default_value(response_property_schema_type)
            ),
            false => mapped_value_name.to_string(),
        }
    }
}

fn is_query_parameter(
    query_parameters: Vec<(String, SchemaType)>,
    mapped_value_name: &str,
) -> bool {
    query_parameters
        .iter()
        .any(|(name, _)| -> bool { name.to_string() == mapped_value_name })
}

fn convert_type_to_default_value(schema_type: SchemaType) -> String {
    match schema_type {
        SchemaType::Boolean => "false".to_string(),
        SchemaType::Integer => "0".to_string(),
        SchemaType::Number => "0.0".to_string(),
        SchemaType::String => "".to_string(),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
    }
}
