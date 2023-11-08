mod create_response_field_object;
mod create_response_field_primitive;

use codegen::Function;
use create_response_field_object::create_response_field_object;
use create_response_field_primitive::create_response_field_primitive;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::{
    generate_workflows::generate_workflow::generate_workflow_response::generate_response_structure::ResponseWithStructName,
    traversal::NestedNode,
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
