use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::parse_specs::OperationSpec;

pub fn create_variable_names(
    iter: std::slice::Iter<'_, OperationSpec>,
    variable_aliases: &mut VariableAliases,
) -> Vec<String> {
    iter.clone()
        .map(|_| variable_aliases.create_alias())
        .collect()
}
