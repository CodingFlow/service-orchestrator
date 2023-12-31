use crate::generate_workflows::generate_workflow::build_service_call_view_data::build_service_operation_lookup_map::ServiceCodeGenerationInfo;
use crate::generate_workflows::generate_workflow::generate_workflow_response::generate_map_response::generate_service_calls::generate_calls::generate_response_variables_assigned;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use codegen::Function;
use std::collections::BTreeMap;

pub fn generate_signature_and_dependencies_variables(
    number_depending: usize,
    dependencies_service_and_operation_names: &Vec<(String, String)>,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    function: &mut Function,
    future_variable_name: &String,
    variable_aliases: &mut VariableAliases,
) {
    match number_depending {
        number if number > 1 => {
            {
                let future_variable_name = future_variable_name;
                let (
                    depending_operation_aliases,
                    depending_future_variable_names,
                    formatted_depending_operation_aliases,
                ) = create_future_signature_data_with_dependencies(
                    dependencies_service_and_operation_names,
                    variable_aliases,
                    generation_infos,
                );

                function.line(format!(
                    "let {} = async {{ tokio::join!({}) }}.then(|({})| async {{",
                    future_variable_name,
                    depending_future_variable_names,
                    formatted_depending_operation_aliases,
                ));

                generate_future_dependencies_variables(
                    dependencies_service_and_operation_names,
                    generation_infos,
                    function,
                    depending_operation_aliases,
                );
            };
        }
        number if number == 1 => {
            let (
                depending_operation_aliases,
                depending_future_variable_names,
                formatted_depending_operation_aliases,
            ) = create_future_signature_data_with_dependencies(
                dependencies_service_and_operation_names,
                variable_aliases,
                generation_infos,
            );

            function.line(format!(
                "let {} = {}.then(|{}| async {{",
                future_variable_name,
                depending_future_variable_names,
                formatted_depending_operation_aliases,
            ));

            generate_future_dependencies_variables(
                dependencies_service_and_operation_names,
                generation_infos,
                function,
                depending_operation_aliases,
            );
        }
        number if number == 0 => {
            function.line(format!("let {} = async {{", future_variable_name));
        }
        _ => {}
    };
}

pub fn create_future_signature_data_with_dependencies(
    depending_service_and_operation_names: &Vec<(String, String)>,
    variable_aliases: &mut VariableAliases,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
) -> (Vec<String>, String, String) {
    let depending_operation_aliases = depending_service_and_operation_names
        .iter()
        .map(|_| variable_aliases.create_alias())
        .collect::<Vec<String>>();

    let depending_future_variable_names = depending_service_and_operation_names
        .iter()
        .map(|service_and_operation_name| generation_infos.get(service_and_operation_name).unwrap())
        .map(|info| info.future_variable_name.to_string())
        .map(|var_name| format!("{}.clone()", var_name))
        .collect::<Vec<String>>()
        .join(",");

    let formatted_depending_operation_aliases = depending_operation_aliases.join(",");
    (
        depending_operation_aliases,
        depending_future_variable_names,
        formatted_depending_operation_aliases,
    )
}

pub fn generate_future_dependencies_variables(
    depending_service_and_operation_names: &Vec<(String, String)>,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    function: &mut Function,
    depending_operation_aliases: Vec<String>,
) {
    let mut depending_operation_alias_iter = depending_operation_aliases.iter();

    for depending_service_and_operation in depending_service_and_operation_names {
        let depending_service_code_generation_info = generation_infos
            .get(depending_service_and_operation)
            .unwrap();

        generate_response_variables_assigned(
            function,
            &depending_service_code_generation_info.response_aliases,
        );

        let depending_operation_alias = depending_operation_alias_iter.next().unwrap();

        function.line(format!("= {}.unwrap();", depending_operation_alias));
    }
}
