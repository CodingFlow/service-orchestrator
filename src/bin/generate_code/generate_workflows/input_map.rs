use std::{collections::BTreeMap, fs};

use serde_json::{Map, Value};

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

pub trait InputMapBehavior {
    fn get_workflow_services_operations_names(
        &self,
        workflow_name: String,
    ) -> Vec<(String, String)>;

    fn create_variable_alias(&mut self, original_name: String) -> Variable;

    fn get_variable_alias(&self, map_to_key: String) -> String;
}

impl InputMapBehavior for InputMap {
    fn get_workflow_services_operations_names(
        &self,
        workflow_name: String,
    ) -> Vec<(String, String)> {
        let services = &self
            .input_map_config
            .get(&workflow_name)
            .unwrap()
            .as_object()
            .unwrap();

        services
            .iter()
            .filter(|(key, _)| -> bool { **key != "response" })
            .flat_map(|(service_name, value)| {
                let operations = (*value).as_object().unwrap();

                operations.iter().map(|(operation_name, operation_value)| {
                    (service_name.to_string(), operation_name.to_string())
                })
            })
            .collect()
    }

    fn create_variable_alias(&mut self, original_name: String) -> Variable {
        Variable {
            original_name: original_name.to_string(),
            alias: self.create_alias(original_name),
        }
    }

    fn get_variable_alias(&self, map_to_key: String) -> String {
        let map_from_value = match self.input_map_config.pointer(&map_to_key) {
            Some(value) => value.as_str().unwrap(),
            None => panic!("No mapped value found for key '{}'", map_to_key),
        };

        match self.alias_lookup.get(map_from_value) {
            Some(alias) => alias.to_string(),
            None => panic!("Alias not found for key '{}'", map_to_key),
        }
    }
}

impl InputMap {
    fn create_alias(&mut self, original_name: String) -> String {
        let new_value = self.last_created_alias + 1;
        let new_alias = format!("a{}", new_value);

        self.last_created_alias = new_value;

        self.alias_lookup
            .insert(original_name, new_alias.to_string());

        new_alias.to_string()
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
        alias_lookup: BTreeMap::new(),
        last_created_alias: 0,
    }
}
