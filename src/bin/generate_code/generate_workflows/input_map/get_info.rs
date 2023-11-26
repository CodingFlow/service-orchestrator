use super::InputMap;
use crate::traversal::{convert_to_nested_node, traverse_nested_node, NestedNode};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

impl InputMap {
    pub fn get_workflow_response_dependencies_ids(
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
            .filter(|dependency_id| self.is_service_name(dependency_id.to_string()))
            .map(|raw_source_key| {
                let source_key = self.parse_source_key(&raw_source_key);

                (source_key.service.unwrap(), source_key.operation.unwrap())
            })
            .collect()
    }

    pub fn get_workflow_services_operations_ids(
        &self,
        workflow_name: String,
    ) -> Vec<(String, String)> {
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

    pub fn get_service_dependencies(
        &self,
        workflow_name: String,
    ) -> BTreeMap<(String, String), Vec<(String, String)>> {
        let services_operations = self.get_workflow_services_operations_nested(workflow_name);

        let mut all_service_property_names = vec![];

        for service_operation in services_operations {
            let mut service_property_names = vec![];

            traverse_nested_node(
                service_operation.1.clone(),
                |parent_node, (me, service_properties)| {
                    let value = parent_node.current;

                    if !value.is_object() && me.is_service_name(value.as_str().unwrap().to_string())
                    {
                        service_properties.push(value.as_str().unwrap().to_string())
                    }
                },
                |_, _, _| {},
                |_, _| {},
                &mut (self, &mut service_property_names),
            );

            all_service_property_names.push((service_operation.0, service_property_names));
        }

        all_service_property_names
            .into_iter()
            .map(|(service_operation_name, property_names)| {
                let mut dependencies_service_names: Vec<(String, String)> = property_names
                    .iter()
                    .map(|raw_source_key| {
                        let source_key = self.parse_source_key(raw_source_key);

                        (source_key.service.unwrap(), source_key.operation.unwrap())
                    })
                    .collect();

                dependencies_service_names.sort_unstable(); // sort to dedup
                dependencies_service_names.dedup();

                (service_operation_name, dependencies_service_names)
            })
            .collect()
    }

    fn get_next_level(&self, value: Map<String, Value>, key: String) -> Map<String, Value> {
        value.get(&key).unwrap().as_object().unwrap().clone()
    }

    fn get_workflow_services_operations_nested(
        &self,
        workflow_name: String,
    ) -> Vec<((String, String), NestedNode<Value>)> {
        self.get_workflow_services_operations(workflow_name)
            .into_iter()
            .map(|(service_operation_name, value)| {
                let nested_value = convert_to_nested_node(
                    value,
                    |item, _| item,
                    |value, _| {
                        if value.is_object() {
                            Some(
                                value
                                    .as_object()
                                    .unwrap()
                                    .iter()
                                    .map(|(_, value)| value.clone())
                                    .collect(),
                            )
                        } else {
                            None
                        }
                    },
                    &mut (),
                );

                (service_operation_name, nested_value)
            })
            .collect()
    }

    fn get_workflow_services_operations(
        &self,
        workflow_name: String,
    ) -> BTreeMap<(String, String), Value> {
        self.get_workflow_services(workflow_name)
            .into_iter()
            .flat_map(|(service_name, value)| {
                value
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(operation_name, value)| {
                        (
                            (service_name.to_string(), operation_name.to_string()),
                            value.clone(),
                        )
                    })
                    .collect::<BTreeMap<(String, String), Value>>()
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
