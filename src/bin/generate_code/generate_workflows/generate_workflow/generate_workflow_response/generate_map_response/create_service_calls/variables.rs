use crate::generate_workflows::input_map::Variable;

use std::collections::BTreeMap;

pub struct VariableAliases {
    alias_lookup: BTreeMap<String, String>,
    last_created_alias_value: u32,
}

impl VariableAliases {
    pub fn new() -> VariableAliases {
        VariableAliases {
            alias_lookup: BTreeMap::new(),
            last_created_alias_value: 0,
        }
    }

    pub fn create_alias(&mut self) -> String {
        let new_alias_value = self.last_created_alias_value + 1;
        self.last_created_alias_value = new_alias_value;

        format!("b{}", new_alias_value.to_string())
    }

    pub fn create_stored_alias(&mut self, original_name: String) -> Variable {
        let new_alias = self.create_alias();

        self.alias_lookup
            .insert(original_name.clone(), new_alias.clone());

        Variable {
            original_name,
            alias: new_alias,
        }
    }

    pub fn get_alias(&self, original_name: String) -> String {
        self.alias_lookup.get(&original_name).unwrap().to_string()
    }
}
