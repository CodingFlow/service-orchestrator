use super::{is_service_name, InputMap, Variable};

#[derive(Debug, Clone)]

pub enum Location {
    Query,
    Path,
    Header,
    Cookie,
    Body,
}

impl InputMap {
    /// Make source value available at specific location.
    pub fn create_variable_alias(
        &mut self,
        namespace: (String, String, Option<String>, Location),
        map_to_key: Vec<String>,
    ) -> Variable {
        let map_pointer = create_map_pointer(
            (namespace.0, namespace.1, namespace.2, Some(namespace.3)),
            &map_to_key,
        );

        Variable {
            original_name: map_to_key.last().unwrap().to_string(),
            alias: self.create_alias(map_pointer),
        }
    }

    /// Get source value location for given destination location.
    pub fn get_variable_alias(
        &self,
        namespace: (String, String, Option<String>, Location),
        map_to_key: Vec<String>,
    ) -> String {
        let map_pointer = create_map_pointer(
            (
                namespace.0.to_string(),
                namespace.1,
                namespace.2,
                Some(namespace.3),
            ),
            &map_to_key,
        );
        let map_from_value = match self.input_map_config.pointer(&map_pointer) {
            Some(value) => value.as_str().unwrap(),
            None => panic!("No mapped value found for key '{}'", map_to_key.join("/")),
        };

        let (workflow_name, _, _, _) = namespace;

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
    (workflow_name, service_name, service_operation_name, location): (
        String,
        String,
        Option<String>,
        Option<Location>,
    ),
    map_to_key: &Vec<String>,
) -> String {
    let mut namespace = match service_operation_name {
        Some(service_operation_name) => vec![
            String::new(),
            workflow_name,
            service_name,
            service_operation_name,
        ],
        None => vec![String::new(), workflow_name, service_name],
    };

    if let Some(location) = location {
        namespace.push(location_to_string(location));
    }

    namespace.push(String::new());

    format!("{}{}", namespace.join("/"), map_to_key.join("/"))
}

fn location_to_string(location: Location) -> String {
    match location {
        Location::Query => "query",
        Location::Path => "path",
        Location::Header => "header",
        Location::Cookie => "cookie",
        Location::Body => "body",
    }
    .to_string()
}
