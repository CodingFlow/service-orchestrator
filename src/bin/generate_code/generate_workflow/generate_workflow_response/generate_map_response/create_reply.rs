use codegen::Function;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::{
    generate_workflow::generate_workflow_response::generate_response_structure::ResponseWithStructName,
    spec_parsing::ParsedSchema, traversal::NestedNode,
};

pub fn create_reply(
    function: &mut Function,
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    input_map: Map<String, Value>,
    query_parameters: Vec<(String, SchemaType)>,
) {
    let (_, response_node) = status_code_struct_name_node.clone();

    function.line(format!(
        "Ok(reply::json(&{} {{",
        response_node.current.struct_name.unwrap() // Top level node always has a struct so can unwrap safely.
    ));

    create_properties(
        status_code_struct_name_node,
        function,
        input_map,
        query_parameters,
    );

    function.line("}))");
}

fn create_properties(
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    function: &mut Function,
    input_map: Map<String, Value>,
    query_parameters: Vec<(String, SchemaType)>,
) {
    let (status_code, struct_name_node) = status_code_struct_name_node;

    for response_property in struct_name_node.children.unwrap() {
        create_response_field(function, response_property, &input_map, &query_parameters);
    }
}

fn create_response_field(
    function: &mut Function,
    struct_name_node: NestedNode<ResponseWithStructName>,
    input_map: &Map<String, Value>,
    query_parameters: &Vec<(String, SchemaType)>,
) {
    match struct_name_node.current.schema.schema_type {
        SchemaType::Array => todo!(),
        SchemaType::Object => {
            create_response_field_object(function, struct_name_node, input_map, query_parameters)
        }
        _ => create_response_field_primitive(
            function,
            struct_name_node.current.schema,
            input_map,
            query_parameters,
        ),
    }
}

fn create_response_field_object(
    function: &mut Function,
    struct_name_node: NestedNode<ResponseWithStructName>,
    input_map: &Map<String, Value>,
    query_parameters: &Vec<(String, SchemaType)>,
) {
    let response_property_schema = struct_name_node.current.schema;
    let property_name = response_property_schema.name.clone().unwrap();
    let mapped_value_map = input_map.get(&property_name).unwrap().as_object().unwrap();

    let property_name = property_name;
    let struct_name = struct_name_node.current.struct_name.unwrap();

    function.line(format!("{}:{} {{", property_name, struct_name));

    if let Some(struct_name_node_children) = struct_name_node.children {
        for child_struct_name_node in struct_name_node_children {
            create_response_field(
                function,
                child_struct_name_node,
                mapped_value_map,
                query_parameters,
            );
        }
    }

    function.line("},");
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
    let response_property_schema_type = response_property_schema_type;
    let mapped_value_name = mapped_value_name;
    match is_query_parameter(query_parameters, mapped_value_name) {
        true => match response_property_schema_type {
            SchemaType::String => format!(
                "{}.get_or_insert({}).to_string()",
                mapped_value_name,
                convert_type_to_default_value(response_property_schema_type)
            ),
            _ => format!(
                "*{}.get_or_insert({})",
                mapped_value_name,
                convert_type_to_default_value(response_property_schema_type)
            ),
        },
        false => mapped_value_name.to_string(),
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
        SchemaType::String => "String::new()".to_string(),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
    }
}
