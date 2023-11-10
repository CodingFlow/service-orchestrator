use codegen::Function;

use crate::{
    generate_workflows::input_map::{InputMap, InputMapBehavior},
    spec_parsing::ParsedSchema,
};

pub fn create_response_field_primitive(
    function: &mut Function,
    response_property: ParsedSchema,
    input_map: &InputMap,
    map_pointer: String,
) {
    let property_name = response_property.name.unwrap();
    let new_map_pointer = format!("{}/{}", map_pointer, property_name);
    let mapped_value_name = input_map.get_variable_alias(new_map_pointer);

    function.line(format!(
        "{}:{},",
        property_name,
        mapped_value_name.to_string(),
    ));
}
