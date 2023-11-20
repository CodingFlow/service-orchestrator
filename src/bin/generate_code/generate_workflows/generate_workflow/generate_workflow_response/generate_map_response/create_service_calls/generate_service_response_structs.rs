use crate::{
    generate_workflows::generate_workflow::generate_workflow_response::{
        generate_response_structs::generate_response_structs,
        generate_response_variables::ServiceResponseAlias,
    },
    traversal::NestedNode,
};
use codegen::Scope;

use super::build_service_operation_lookup_map::ServiceCodeGenerationInfo;

pub fn generate_service_response_structs(
    scope: &mut Scope,
    generation_infos_with_ids: Vec<(
        (std::string::String, std::string::String),
        ServiceCodeGenerationInfo,
    )>,
) {
    let response_specs: Vec<NestedNode<ServiceResponseAlias>> = generation_infos_with_ids
        .iter()
        .map(|(_, info)| info.response_aliases.clone())
        .collect();

    generate_response_structs(response_specs, scope);
}
