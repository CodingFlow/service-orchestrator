use crate::traversal::{convert_to_nested_node, traverse_nested_node, NestedNode};
use serde_json::{Map, Value};
use std::{collections::BTreeMap, fs};

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
    fn get_workflow_response_dependencies_ids(
        &self,
        workflow_name: String,
    ) -> Vec<(String, String)>;

    fn get_workflow_services_operations_ids(&self, workflow_name: String) -> Vec<(String, String)>;

    fn get_service_dependencies(
        &self,
        workflow_name: String,
    ) -> BTreeMap<String, Vec<(String, String)>>;

    fn create_variable_alias(
        &mut self,
        namespace: (String, String, Option<String>),
        map_to_key: Vec<String>,
    ) -> Variable;

    fn get_variable_alias(
        &self,
        namespace: (String, String, Option<String>),
        map_to_key: Vec<String>,
    ) -> String;
}

impl InputMapBehavior for InputMap {
    fn get_workflow_response_dependencies_ids(
        &self,
        workflow_name: String,
    ) -> Vec<(String, String)> {
        let response_properties = self.get_workflow_response_properties(workflow_name.to_string());

        let mut dependencies_properties = vec![];

        for property in response_properties {
            traverse_nested_node(
                property.clone(),
                |parent_node, dependencies_ids| {
                    let (_, value) = parent_node.current;

                    if !value.is_object() {
                        dependencies_ids.push(value.as_str().unwrap().to_string())
                    }
                },
                |_, _, _| {},
                |_, _| {},
                &mut dependencies_properties,
            );
        }

        dependencies_properties
            .into_iter()
            .filter(|dependency_id| is_service_name(dependency_id.to_string()))
            .map(|property_name| {
                let split = &mut property_name.split("/");
                (
                    split.nth(0).unwrap().to_string(),
                    split.nth(0).unwrap().to_string(),
                )
            })
            .collect()
    }

    fn get_workflow_services_operations_ids(&self, workflow_name: String) -> Vec<(String, String)> {
        self.get_all_workflow_services(workflow_name)
            .iter()
            .flat_map(|(service_name, operations)| {
                operations
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(operation_id, _)| (service_name.to_string(), operation_id.to_string()))
            })
            .collect()
    }

    fn get_service_dependencies(
        &self,
        workflow_name: String,
    ) -> BTreeMap<String, Vec<(String, String)>> {
        let services = self.get_workflow_services_nested(workflow_name);

        let mut all_service_property_names = vec![];

        for service in services {
            let mut service_property_names = vec![];
            traverse_nested_node(
                service.clone(),
                |parent_node, service_properties| {
                    let (_, value) = parent_node.current;

                    if !value.is_object() && is_service_name(value.as_str().unwrap().to_string()) {
                        service_properties.push(value.as_str().unwrap().to_string())
                    }
                },
                |_, _, _| {},
                |_, _| {},
                &mut service_property_names,
            );

            all_service_property_names.push((service.current.0, service_property_names));
        }

        all_service_property_names
            .into_iter()
            .map(|(service_name, property_names)| {
                let mut dependent_service_names: Vec<(String, String)> = property_names
                    .iter()
                    .map(|name| {
                        let split = &mut name.split("/");
                        (
                            split.nth(0).unwrap().to_string(),
                            split.nth(0).unwrap().to_string(),
                        )
                    })
                    .collect();

                dependent_service_names.sort_unstable();
                dependent_service_names.dedup();

                (service_name, dependent_service_names)
            })
            .collect()
    }

    fn create_variable_alias(
        &mut self,
        (workflow_name, service_name, service_operation_name): (String, String, Option<String>),
        map_to_key: Vec<String>,
    ) -> Variable {
        let namespace = match service_operation_name {
            Some(service_operation_name) => format!(
                "/{}/{}/{}/",
                workflow_name, service_name, service_operation_name
            ),
            None => format!("/{}/{}/", workflow_name, service_name),
        };

        let map_pointer = format!("{}{}", namespace, map_to_key.join("/"));

        Variable {
            original_name: map_to_key.last().unwrap().to_string(),
            alias: self.create_alias(map_pointer),
        }
    }

    fn get_variable_alias(
        &self,
        (workflow_name, service_name, service_operation_name): (String, String, Option<String>),
        map_to_key: Vec<String>,
    ) -> String {
        let namespace = match service_operation_name {
            Some(service_operation_name) => format!(
                "/{}/{}/{}/",
                workflow_name, service_name, service_operation_name
            ),
            None => format!("/{}/{}/", workflow_name, service_name),
        };

        let map_pointer = format!("{}{}", namespace, map_to_key.join("/"));
        let map_from_value = match self.input_map_config.pointer(&map_pointer) {
            Some(value) => value.as_str().unwrap(),
            None => panic!("No mapped value found for key '{}'", map_to_key.join("/")),
        };

        let alias_lookup_value = match is_service_name(map_from_value.to_string()) {
            true => format!("/{}/{}", workflow_name, map_from_value),
            false => format!("/{}/response/{}", workflow_name, map_from_value.to_string()),
        };

        match self.alias_lookup.get(&alias_lookup_value) {
            Some(alias) => alias.to_string(),
            None => panic!("Alias not found for key '{}'", map_to_key.join("/")),
        }
    }
}

fn is_service_name(name: String) -> bool {
    let second_part = name.split("/").nth(1);

    second_part.is_some() && second_part.unwrap() != "response"
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

    fn get_next_level(&self, value: Map<String, Value>, key: String) -> Map<String, Value> {
        value.get(&key).unwrap().as_object().unwrap().clone()
    }

    fn get_workflow_services_nested(
        &self,
        workflow_name: String,
    ) -> Vec<NestedNode<(String, Value)>> {
        self.get_workflow_services(workflow_name)
            .into_iter()
            .map(|item| {
                convert_to_nested_node(
                    item,
                    |item, _| item,
                    |(_, value), _| {
                        if value.is_object() {
                            Some(
                                value
                                    .as_object()
                                    .unwrap()
                                    .iter()
                                    .map(|(key, value)| (key.to_string(), value.clone()))
                                    .collect(),
                            )
                        } else {
                            None
                        }
                    },
                    &mut (),
                )
            })
            .collect()
    }

    fn get_workflow_services(&self, workflow_name: String) -> Map<String, Value> {
        let all_services = self.get_all_workflow_services(workflow_name);

        all_services
            .iter()
            .filter(|(key, _)| -> bool { **key != "response" })
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect()
    }

    fn get_all_workflow_services(&self, workflow_name: String) -> Map<String, Value> {
        self.get_next_level(
            self.input_map_config.as_object().unwrap().clone(),
            workflow_name,
        )
    }

    fn get_workflow_response_properties(
        &self,
        workflow_name: String,
    ) -> Vec<NestedNode<(String, Value)>> {
        let map = self.get_next_level(
            self.get_all_workflow_services(workflow_name),
            "response".to_string(),
        );

        map.into_iter()
            .map(|item| {
                convert_to_nested_node(
                    item,
                    |item, _| item,
                    |(_, value), _| {
                        if value.is_object() {
                            Some(
                                value
                                    .as_object()
                                    .unwrap()
                                    .iter()
                                    .map(|(key, value)| (key.to_string(), value.clone()))
                                    .collect(),
                            )
                        } else {
                            None
                        }
                    },
                    &mut (),
                )
            })
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
