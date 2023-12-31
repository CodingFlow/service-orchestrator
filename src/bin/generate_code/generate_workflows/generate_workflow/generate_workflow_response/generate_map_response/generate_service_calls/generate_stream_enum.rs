use codegen::Scope;

use crate::generate_workflows::generate_workflow::build_service_call_view_data::build_service_operation_lookup_map::ServiceCodeGenerationInfo;

pub fn generate_stream_enum(
    scope: &mut Scope,
    ordered_generation_infos_with_id: Vec<((String, String), ServiceCodeGenerationInfo)>,
) {
    let struct_names_and_enums: Vec<(String, String)> = ordered_generation_infos_with_id
        .iter()
        .map(|(_, info)| {
            (
                info.enum_name.to_string(),
                info.response_aliases.current.variable_alias.to_string(),
            )
        })
        .collect();

    let new_enum = scope.new_enum("Message");

    for (enum_name, response_struct_name) in struct_names_and_enums {
        let variant = new_enum.new_variant(&enum_name);

        variant.tuple(&format!("Result<{}, StatusCode>", response_struct_name));
    }
}
