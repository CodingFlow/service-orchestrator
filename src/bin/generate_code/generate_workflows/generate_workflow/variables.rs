pub struct VariableAliases {
    last_created_alias_value: u32,
}

impl VariableAliases {
    pub fn new() -> VariableAliases {
        VariableAliases {
            last_created_alias_value: 0,
        }
    }

    pub fn create_alias(&mut self) -> String {
        let new_alias_value = self.last_created_alias_value + 1;
        self.last_created_alias_value = new_alias_value;

        format!("b{}", new_alias_value.to_string())
    }
}
