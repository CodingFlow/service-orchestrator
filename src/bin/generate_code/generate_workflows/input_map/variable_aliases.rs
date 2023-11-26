use super::InputMap;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Location {
    Query,
    Path,
    Header,
    Cookie,
    Body,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AliasKey((String, String, Option<String>, Location), Vec<String>);

#[derive(Debug, Clone)]
pub struct Variable {
    pub original_name: String,
    pub alias: String,
}

impl InputMap {
    /// Make source value available at specific location.
    pub fn create_variable_alias(
        &mut self,
        namespace: (String, String, Option<String>, Location),
        map_to_key: Vec<String>,
    ) -> Variable {
        let map_to_key = map_to_key.to_vec();
        let alias_key = AliasKey(namespace, map_to_key.clone());

        Variable {
            original_name: map_to_key.last().unwrap().to_string(),
            alias: self.create_alias(alias_key),
        }
    }

    /// Get source value location for given destination location.
    pub fn get_variable_alias(
        &self,
        namespace: (String, String, Option<String>, Location),
        destination_key: Vec<String>,
    ) -> String {
        let map_pointer = create_map_pointer(namespace.clone(), &destination_key);
        let source_key_raw = match self.input_map_config_pointer.pointer(&map_pointer) {
            Some(value) => value.as_str().unwrap(),
            None => panic!(
                "No mapped value found for key '{}'",
                destination_key.join("/")
            ),
        };

        let (workflow_name, _, _, _) = namespace;

        self.get_alias(source_key_raw, workflow_name)
    }

    fn get_alias(&self, source_key_raw: &str, workflow_name: String) -> String {
        let mut split = source_key_raw.split(":");
        let namespace_part = split.next().unwrap();
        let key_part = split.next().unwrap();

        let mut split = namespace_part.split("/");

        let alias_key = match self.is_service_name(source_key_raw.to_string()) {
            true => AliasKey(
                (
                    workflow_name,
                    split.next().unwrap().to_string(),
                    Some(split.next().unwrap().to_string()),
                    string_to_location(split.next().unwrap()),
                ),
                key_part.split("/").map(String::from).collect(),
            ),
            false => AliasKey(
                (
                    workflow_name,
                    "response".to_string(),
                    None,
                    string_to_location(split.next().unwrap()),
                ),
                key_part.split("/").map(String::from).collect(),
            ),
        };

        match self.alias_lookup.get(&alias_key) {
            Some(alias) => alias.to_string(),
            None => panic!("Alias not found for source key '{}'", source_key_raw),
        }
    }

    fn create_alias(&mut self, alias_key: AliasKey) -> String {
        let new_value = self.last_created_alias + 1;
        let new_alias = format!("a{}", new_value);

        self.last_created_alias = new_value;

        self.alias_lookup.insert(alias_key, new_alias.to_string());

        new_alias.to_string()
    }
}

fn create_map_pointer(
    (workflow_name, service_name, service_operation_name, location): (
        String,
        String,
        Option<String>,
        Location,
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

    namespace.push(location_to_string(location));
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

fn string_to_location(string: &str) -> Location {
    match string {
        "query" => Location::Query,
        "path" => Location::Path,
        "header" => Location::Header,
        "cookie" => Location::Cookie,
        "body" => Location::Body,
        _ => panic!("Unsupported location string used: {}", string),
    }
}
