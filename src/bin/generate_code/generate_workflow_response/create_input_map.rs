use std::fs;

use serde_json::{Map, Value};

pub fn create_input_map() -> Map<String, Value> {
    let file = match fs::File::open("./src/workflow_mapping.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read workflow mapping configuration file."),
    };
    let config: serde_json::Value = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse workflow mapping configuration file."),
    };

    let workflow_config = config.get("Workflow A").unwrap();
    let response = workflow_config
        .get("response")
        .unwrap()
        .as_object()
        .unwrap();

    response.clone()
}
