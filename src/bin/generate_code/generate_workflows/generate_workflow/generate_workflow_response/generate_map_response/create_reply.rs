mod create_response_field_object;
mod create_response_field_primitive;

use codegen::Function;
use create_response_field_object::create_response_field_object;
use create_response_field_primitive::create_response_field_primitive;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::{
    generate_workflows::{
        extract_request_parameters_from_spec::RequestParameter,
        generate_workflow::generate_workflow_response::generate_response_structure::ResponseWithStructName,
        input_map::InputMap,
    },
    traversal::NestedNode,
};

pub fn create_reply(
    function: &mut Function,
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    map_object: Map<String, Value>,
    query_parameters: Vec<RequestParameter>,
    input_map: &InputMap,
) {
    let (_, response_node) = status_code_struct_name_node.clone();

    function.line(format!(
        "Ok(reply::json(&{} {{",
        response_node.current.struct_name.unwrap() // Top level node always has a struct so can unwrap safely.
    ));

    create_properties(
        status_code_struct_name_node,
        function,
        map_object,
        query_parameters,
        input_map,
    );

    function.line("}))");
}

fn create_properties(
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    function: &mut Function,
    map_object: Map<String, Value>,
    query_parameters: Vec<RequestParameter>,
    input_map: &InputMap,
) {
    // TODO: Handle different status codes.

    let (status_code, struct_name_node) = status_code_struct_name_node;

    // TODO: Update [traverse_nested_type] and use it. Need to support after children action and output from child action.
    for response_property in struct_name_node.children.unwrap() {
        create_response_field(
            function,
            response_property,
            &map_object,
            query_parameters.to_vec(),
            input_map,
        );
    }
}

fn create_response_field(
    function: &mut Function,
    struct_name_node: NestedNode<ResponseWithStructName>,
    map_object: &Map<String, Value>,
    query_parameters: Vec<RequestParameter>,
    input_map: &InputMap,
) {
    match struct_name_node.current.schema.schema_type {
        SchemaType::Array => todo!(),
        SchemaType::Object => create_response_field_object(
            function,
            struct_name_node,
            map_object,
            query_parameters,
            input_map,
        ),
        _ => create_response_field_primitive(
            function,
            struct_name_node.current.schema,
            map_object,
            query_parameters,
            input_map,
        ),
    }
}
