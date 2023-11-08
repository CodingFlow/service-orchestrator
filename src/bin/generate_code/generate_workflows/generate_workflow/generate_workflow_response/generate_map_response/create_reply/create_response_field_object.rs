use codegen::Function;
use oas3::spec::SchemaType;
use serde_json::{Map, Value};

use crate::{traversal::NestedNode, generate_workflows::generate_workflow::generate_workflow_response::generate_response_structure::ResponseWithStructName};

use super::create_response_field;

pub fn create_response_field_object(
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
