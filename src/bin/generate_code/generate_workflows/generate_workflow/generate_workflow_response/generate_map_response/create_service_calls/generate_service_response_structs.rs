use crate::traversal::NestedNode;
use codegen::Scope;
use generate_response_structs::generate_response_structs;

use super::{
    build_service_operation_lookup_map::{ServiceCodeGenerationInfo, ServiceResponseAlias},
    generate_response_structs,
};

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
