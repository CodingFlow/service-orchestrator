use codegen::Function;
use crate::generate_workflows::generate_workflow::build_service_call_view_data::build_service_operation_lookup_map::ServiceCodeGenerationInfo;

pub fn generate_streams(
    function: &mut Function,
    generation_infos_with_ids: Vec<((String, String), ServiceCodeGenerationInfo)>,
) {
    for (_, generation_info) in &generation_infos_with_ids {
        function.line(format!(
            "let {} = futures::FutureExt::into_stream({}.clone()).map(Message::{});",
            generation_info.stream_variable_name,
            generation_info.future_variable_name,
            generation_info.enum_name
        ));
    }

    let mut iter = generation_infos_with_ids
        .iter()
        .map(|(_, info)| info.stream_variable_name.to_string());

    let first_stream_variable = iter.next().unwrap();
    let formatted_merged_streams = iter
        .map(|stream_variable_name| format!(".merge({})", stream_variable_name))
        .collect::<Vec<String>>()
        .join("");

    let all_formatted_merged_streams =
        format!("{}{}", first_stream_variable, formatted_merged_streams);

    function.line(format!("let merged = {};", all_formatted_merged_streams));
}
