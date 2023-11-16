use std::{collections::BTreeMap, fmt::format, fs};

use serde_json::Value;

use crate::traversal::traverse_nested_type;

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

    fn get_service_dependencies(
        &self,
        workflow_name: String,
    ) -> BTreeMap<String, Vec<(String, String)>>;

    fn create_variable_alias(&mut self, original_name: String) -> Variable;

    fn get_variable_alias(&self, map_to_key: String) -> String;
}

impl InputMapBehavior for InputMap {
    fn get_workflow_services_operations_names(
        &self,
        workflow_name: String,
    ) -> Vec<(String, String)> {
        let services = self.get_workflow_services(workflow_name);

        services
            .iter()
            .flat_map(|(service_name, value)| {
                let operations = (*value).as_object().unwrap();

                operations.iter().map(|(operation_name, operation_value)| {
                    (service_name.to_string(), operation_name.to_string())
                })
            })
            .collect()
    }

    fn get_service_dependencies(
        &self,
        workflow_name: String,
    ) -> BTreeMap<String, Vec<(String, String)>> {
        let services = self.get_workflow_services(workflow_name);
        let service_names: Vec<String> =
            services.iter().map(|(name, _)| name.to_string()).collect();

        let mut all_service_property_names = vec![];

        for service in services {
            let mut service_property_names = vec![];
            traverse_nested_type(
                service.clone(),
                |(_, value), (service_properties, service_names)| {
                    if !value.is_object()
                        && is_service(value.as_str().unwrap().to_string(), service_names.to_vec())
                    {
                        service_properties.push(value.as_str().unwrap().to_string())
                    }
                },
                |_, _, _| {},
                |(_, value)| match value.is_object() {
                    true => Some(
                        value
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(key, value)| (key.to_string(), value.clone()))
                            .collect(),
                    ),
                    false => None,
                },
                &mut (&mut service_property_names, &service_names),
            );

            all_service_property_names.push((service.0, service_property_names));
        }

        all_service_property_names
            .into_iter()
            .map(|(service_name, property_names)| {
                let dependent_service_names: Vec<(String, String)> = property_names
                    .iter()
                    .map(|name| {
                        let split = &mut name.split("/");
                        (
                            split.nth(0).unwrap().to_string(),
                            split.nth(0).unwrap().to_string(),
                        )
                    })
                    .collect();

                (service_name, dependent_service_names)
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

        let workflow_name = map_to_key.split('/').nth(1).unwrap();
        let services = self.get_workflow_services(workflow_name.to_string());
        let service_names: Vec<String> =
            services.iter().map(|(name, _)| name.to_string()).collect();
        let split_map_from_value = &mut map_from_value.split('/');
        let first_part = split_map_from_value.nth(0).unwrap();

        let alias_lookup_value = match is_service(first_part.to_string(), service_names) {
            true => format!("/{}/{}", workflow_name, map_from_value),
            false => map_from_value.to_string(),
        };

        match self.alias_lookup.get(&alias_lookup_value) {
            Some(alias) => alias.to_string(),
            None => panic!("Alias not found for key '{}'", map_to_key),
        }
    }
}

fn is_service(name: String, service_names: Vec<String>) -> bool {
    let first_part = name.split("/").nth(0).unwrap();
    service_names.contains(&first_part.to_string())
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

    fn get_workflow_services(&self, workflow_name: String) -> serde_json::Map<String, Value> {
        self.input_map_config
            .get(&workflow_name)
            .unwrap()
            .as_object()
            .unwrap()
            .iter()
            .filter(|(key, _)| -> bool { **key != "response" })
            .map(|(key, value)| (key.to_string(), value.clone()))
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
        alias_lookup: BTreeMap::new(),
        last_created_alias: 0,
    }
}
