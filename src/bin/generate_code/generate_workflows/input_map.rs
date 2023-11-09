use std::{collections::BTreeMap, fs};

use serde_json::{Map, Value};

pub struct InputMap {
    input_map_config: Map<String, Value>,
}

pub trait InputMapBehavior {
    fn get_workflow_response(&self, workflow_name: String) -> Map<String, Value>;
    fn get_all_workflows(&self) -> BTreeMap<String, Map<String, Value>>;
    fn get_all_services_for_workflow(&self, workflow_name: String) -> Map<String, Value>;
}

impl InputMapBehavior for InputMap {
    fn get_workflow_response(&self, workflow_name: String) -> Map<String, Value> {
        let workflow = self
            .input_map_config
            .get(&workflow_name)
            .unwrap()
            .as_object()
            .unwrap();

        workflow
            .get("response")
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
    }

    fn get_all_workflows(&self) -> BTreeMap<String, Map<String, Value>> {
        self.input_map_config
            .clone()
            .into_iter()
            .map(|(key, value)| -> (String, Map<String, Value>) {
                (key, value.as_object().unwrap().clone())
            })
            .collect()
    }

    fn get_all_services_for_workflow(&self, workflow_name: String) -> Map<String, Value> {
        self.input_map_config
            .get(&workflow_name)
            .unwrap()
            .as_object()
            .unwrap()
            .iter()
            .filter(|(key, value)| -> bool { **key != "response" })
            .map(|(key, value)| -> (String, Value) { (*key, *value) })
            .collect()
    }
}

pub fn create_input_map() -> InputMap {
    let file = match fs::File::open("./src/workflow_mapping.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read workflow mapping configuration file."),
    };
    let config = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse workflow mapping configuration file."),
    };

    InputMap {
        input_map_config: config,
    }
}
