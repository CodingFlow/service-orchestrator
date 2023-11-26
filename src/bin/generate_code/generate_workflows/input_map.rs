mod get_info;
mod variable_aliases;

use self::variable_aliases::string_to_location;
use self::variable_aliases::AliasKey;
pub use self::variable_aliases::Location;
pub use self::variable_aliases::Variable;
use serde_json::Value;
use std::collections::BTreeMap;

pub struct InputMap {
    input_map_config: Value,
    alias_lookup: BTreeMap<AliasKey, String>,
    last_created_alias: u32,
}

struct SourceKey {
    service: Option<String>,
    operation: Option<String>,
    location: Location,
    property_path: Vec<String>,
}

impl InputMap {
    fn is_service_name(&self, raw_source_key: String) -> bool {
        let source_key = self.parse_source_key(&raw_source_key);

        source_key.service.is_some()
    }

    fn parse_source_key(&self, raw_source_key: &str) -> SourceKey {
        let mut colon_split = raw_source_key.split(":");
        let namespace_part = colon_split.next().unwrap();
        let key_part = colon_split.next().unwrap();

        let mut namespace_split = namespace_part.rsplit("/").map(str::to_string);
        let location = string_to_location(&namespace_split.next().unwrap());
        let operation = namespace_split.next();
        let service = namespace_split.next();

        let property_path = key_part.split("/").map(str::to_string).collect();

        SourceKey {
            service,
            operation,
            location,
            property_path,
        }
    }
}

pub fn create_input_map() -> InputMap {
    let content = match std::fs::read_to_string("./src/workflow_mapping.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read workflow mapping configuration file."),
    };

    let config: Value = match serde_yaml::from_str(&content) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse workflow mapping configuration file."),
    };

    InputMap {
        input_map_config: config,
        alias_lookup: BTreeMap::new(),
        last_created_alias: 0,
    }
}
