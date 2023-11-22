use super::{is_service_name, InputMap, Variable};

impl InputMap {
    pub fn create_variable_alias(
        &mut self,
        namespace: (String, String, Option<String>),
        map_to_key: Vec<String>,
    ) -> Variable {
        let map_pointer = create_map_pointer(namespace, &map_to_key);

        Variable {
            original_name: map_to_key.last().unwrap().to_string(),
            alias: self.create_alias(map_pointer),
        }
    }

    pub fn get_variable_alias(
        &self,
        namespace: (String, String, Option<String>),
        map_to_key: Vec<String>,
    ) -> String {
        let map_pointer = create_map_pointer(namespace.clone(), &map_to_key);
        let map_from_value = match self.input_map_pointer_lookup.pointer(&map_pointer) {
            Some(value) => value.as_str().unwrap(),
            None => panic!("No mapped value found for key '{}'", map_to_key.join("/")),
        };

        let (workflow_name, _, _) = namespace;

        let alias_lookup_value = match is_service_name(map_from_value.to_string()) {
            true => format!("/{}/{}", workflow_name, map_from_value),
            false => format!("/{}/response/{}", workflow_name, map_from_value.to_string()),
        };

        match self.alias_lookup.get(&alias_lookup_value) {
            Some(alias) => alias.to_string(),
            None => panic!("Alias not found for key '{}'", map_to_key.join("/")),
        }
    }

    fn create_alias(&mut self, original_name: String) -> String {
        let new_value = self.last_created_alias + 1;
        let new_alias = format!("a{}", new_value);

        self.last_created_alias = new_value;

        self.alias_lookup
            .insert(original_name, new_alias.to_string());

        new_alias.to_string()
    }
}

fn create_map_pointer(
    (workflow_name, service_name, service_operation_name): (String, String, Option<String>),
    map_to_key: &Vec<String>,
) -> String {
    let namespace = match service_operation_name {
        Some(service_operation_name) => format!(
            "/{}/{}/{}/",
            workflow_name, service_name, service_operation_name
        ),
        None => format!("/{}/{}/", workflow_name, service_name),
    };

    format!("{}{}", namespace, map_to_key.join("/"))
}
