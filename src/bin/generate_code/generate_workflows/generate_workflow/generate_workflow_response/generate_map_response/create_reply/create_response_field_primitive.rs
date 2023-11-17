use codegen::Function;

use crate::{
    generate_workflows::input_map::{InputMap, InputMapBehavior},
    parse_specs::parse_schema::ParsedSchema,
};

pub fn create_response_field_primitive(
    function: &mut Function,
    response_property: ParsedSchema,
    input_map: &InputMap,
    map_pointer: String,
) {
    let property_name = response_property.name.unwrap();
    let mut split = map_pointer.split("/");
    let workflow_name = split.nth(1).unwrap();
    let service_name = split.nth(0).unwrap();
    let mut path: Vec<String> = split.map(|name| name.to_string()).collect();

    path.push(property_name.to_string());

    let mapped_value_name = input_map.get_variable_alias(
        (workflow_name.to_string(), service_name.to_string(), None),
        path,
    );

    function.line(format!(
        "{}:{},",
        property_name,
        mapped_value_name.to_string(),
    ));
}
