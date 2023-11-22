use crate::{
    generate_workflows::generate_workflow::{
        build_service_call_view_data::{
            build_service_operation_lookup_map::ServiceCodeGenerationInfo,
            generate_response_variables::ResponseAlias,
        },
        generate_workflow_response::generate_response_structs::generate_response_structs,
    },
    traversal::NestedNode,
};
use codegen::Scope;

pub fn generate_service_response_structs(
    scope: &mut Scope,
    generation_infos_with_ids: Vec<(
        (std::string::String, std::string::String),
        ServiceCodeGenerationInfo,
    )>,
) {
    let response_specs: Vec<NestedNode<ResponseAlias>> = generation_infos_with_ids
        .iter()
        .map(|(_, info)| info.response_aliases.clone())
        .collect();

    generate_response_structs(response_specs, scope);
}
