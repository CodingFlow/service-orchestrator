mod create_response_field_object;
mod create_response_field_primitive;

use codegen::Function;
use create_response_field_object::create_response_field_object;
use create_response_field_primitive::create_response_field_primitive;
use oas3::spec::SchemaType;

use crate::{
    generate_workflows::{
        add_variable_aliases_to_request_parameters::RequestParameter,
        generate_workflow::generate_workflow_response::generate_response_structure::ResponseWithStructName,
        input_map::InputMap,
    },
    traversal::NestedNode,
};

pub fn create_reply(
    function: &mut Function,
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    query_parameters: Vec<(RequestParameter)>,
    input_map: &InputMap,
    workflow_name: String,
) {
    let (_, response_node) = status_code_struct_name_node.clone();

    function.line(format!(
        "Ok(reply::json(&{} {{",
        response_node.current.struct_name.unwrap() // Top level node always has a struct so can unwrap safely.
    ));

    create_properties(
        status_code_struct_name_node,
        function,
        query_parameters,
        input_map,
        workflow_name,
    );

    function.line("}))");
}

fn create_properties(
    status_code_struct_name_node: (String, NestedNode<ResponseWithStructName>),
    function: &mut Function,
    query_parameters: Vec<RequestParameter>,
    input_map: &InputMap,
    workflow_name: String,
) {
    // TODO: Handle different status codes.

    let (status_code, struct_name_node) = status_code_struct_name_node;

    // TODO: Update [traverse_nested_type] and use it. Need to support after children action and output from child action.
    for response_property in struct_name_node.children.unwrap() {
        create_response_field(
            function,
            response_property,
            query_parameters.to_vec(),
            input_map,
            format!("/{}/response", workflow_name),
        );
    }
}

fn create_response_field(
    function: &mut Function,
    struct_name_node: NestedNode<ResponseWithStructName>,
    query_parameters: Vec<RequestParameter>,
    input_map: &InputMap,
    map_pointer: String,
) {
    match struct_name_node.current.schema.schema_type {
        SchemaType::Array => todo!(),
        SchemaType::Object => create_response_field_object(
            function,
            struct_name_node,
            query_parameters,
            input_map,
            map_pointer,
        ),
        _ => create_response_field_primitive(
            function,
            struct_name_node.current.schema,
            input_map,
            map_pointer,
        ),
    }
}
