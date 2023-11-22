mod get_info;
pub mod variable_aliases;

use serde_json::Value;
use std::collections::BTreeMap;

pub struct InputMap {
    input_map_config: Value,
    alias_lookup: BTreeMap<String, String>,
    last_created_alias: u32,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub original_name: String,
    pub alias: String,
}

fn is_service_name(map_key: String) -> bool {
    let third_part = map_key.split("/").nth(2);

    third_part.is_some() && third_part.unwrap() != "response"
}

pub fn create_input_map() -> InputMap {
    let content = match std::fs::read_to_string("./src/workflow_mapping.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read workflow mapping configuration file."),
    };

    let config: Value = match serde_yaml::from_str(&content) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse workflow mapping configuration file."),
    };

    InputMap {
        input_map_config: config,
        alias_lookup: BTreeMap::new(),
        last_created_alias: 0,
    }
}
