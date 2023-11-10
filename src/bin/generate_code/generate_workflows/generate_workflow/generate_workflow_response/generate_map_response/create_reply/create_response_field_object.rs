use codegen::Function;

use crate::{
    generate_workflows::{
        extract_request_parameters_from_spec::RequestParameter,
        generate_workflow::generate_workflow_response::generate_response_structure::ResponseWithStructName,
        input_map::InputMap,
    },
    traversal::NestedNode,
};

use super::create_response_field;

pub fn create_response_field_object(
    function: &mut Function,
    struct_name_node: NestedNode<ResponseWithStructName>,
    query_parameters: Vec<RequestParameter>,
    input_map: &InputMap,
    map_pointer: String,
) {
    let response_property_schema = struct_name_node.current.schema;
    let property_name = response_property_schema.name.clone().unwrap();

    let new_map_pointer = format!("{}/{}", map_pointer, property_name);

    let struct_name = struct_name_node.current.struct_name.unwrap();

    function.line(format!("{}:{} {{", property_name, struct_name));

    if let Some(struct_name_node_children) = struct_name_node.children {
        for child_struct_name_node in struct_name_node_children {
            create_response_field(
                function,
                child_struct_name_node,
                query_parameters.to_vec(),
                input_map,
                new_map_pointer.clone(),
            );
        }
    }

    function.line("},");
}
