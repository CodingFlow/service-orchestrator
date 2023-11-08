use std::fs;

use serde_json::{Map, Value};

pub struct InputMap {
    workflow_config: Map<String, Value>,
}

trait InputMapBehavior {
    fn get_mapped_value(
        &self,
        target_config_name: String,
        operation_name: Option<String>,
    ) -> String;
}

impl InputMapBehavior for InputMap {
    fn get_mapped_value(
        &self,
        target_config_name: String,
        operation_name: Option<String>,
    ) -> String {
        "".to_string()
    }
}

pub fn create_input_map(workflow_name: String) -> Map<String, Value> {
    let a = InputMap {
        workflow_config: Map::new(),
    };
    get_workflow_map(workflow_name)
}

fn get_workflow_map(workflow_name: String) -> Map<String, Value> {
    let file = match fs::File::open("./src/workflow_mapping.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read workflow mapping configuration file."),
    };
    let config: serde_json::Value = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse workflow mapping configuration file."),
    };

    let workflow_config = config.get(workflow_name).unwrap();
    let response = workflow_config
        .get("response")
        .unwrap()
        .as_object()
        .unwrap();

    response.clone()
}
