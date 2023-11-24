mod get_info;
mod variable_aliases;

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

impl InputMap {
    fn is_service_name(&self, map_key: String) -> bool {
        let mut split = map_key.split(":");
        let namespace_part = split.next().unwrap();
        let split = namespace_part.split("/");

        match split.count().cmp(&1) {
            std::cmp::Ordering::Less => panic!("Unexpectedly got less than one part in namespace"),
            std::cmp::Ordering::Equal => false,
            std::cmp::Ordering::Greater => true,
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
