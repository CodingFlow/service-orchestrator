mod generate_signature_and_dependencies_variables;

use crate::generate_workflows::generate_workflow::variables::VariableAliases;

use super::super::build_service_operation_lookup_map::ServiceCodeGenerationInfo;
use codegen::Function;
use generate_signature_and_dependencies_variables::generate_signature_and_dependencies_variables;
use std::collections::BTreeMap;

pub fn generate_futures(
    function: &mut Function,
    (generation_infos, ordered_generation_infos): &(
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
    variable_aliases: &mut VariableAliases,
) {
    function.line("let client = Client::new();");

    for (_, service_code_generation_info) in ordered_generation_infos.iter().rev() {
        generate_future(
            service_code_generation_info,
            &generation_infos,
            function,
            variable_aliases,
        );
    }
}

fn generate_future(
    service_code_generation_info: &ServiceCodeGenerationInfo,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    function: &mut Function,
    variable_aliases: &mut VariableAliases,
) {
    let ServiceCodeGenerationInfo {
        future_variable_name,
        response_aliases,
        depending_service_names,
        request,
        service_url,
        ..
    } = service_code_generation_info;

    let number_depending = depending_service_names.len();

    generate_signature_and_dependencies_variables(
        number_depending,
        depending_service_names,
        generation_infos,
        function,
        future_variable_name,
        variable_aliases,
    );

    function.line("client");

    let path_iter = request.path.iter();
    let path = path_iter
        .clone()
        .map(|path_part| match path_part.alias.clone() {
            Some(_) => "{}".to_string(),
            None => path_part.name.to_string(),
        })
        .collect::<Vec<String>>()
        .join("/");

    let path_parameters = path_iter
        .filter(|path_part| (*path_part).alias.is_some())
        .map(|path_part| path_part.alias.clone().unwrap())
        .collect::<Vec<String>>()
        .join(",");

    function.line(format!(
        r#".{}(format!("{}{}", {}))"#,
        request.method.to_string().to_lowercase(),
        service_url,
        path,
        path_parameters
    ));

    let query_parameters = request
        .query
        .iter()
        .map(|(name, alias)| format!(r#"("{}", {})"#, name, alias))
        .collect::<Vec<String>>()
        .join(",");

    if query_parameters.len() > 0 {
        function.line(format!(r#".query(&[{}])"#, query_parameters));
    }

    function.line(".send()").line(".await").line(".unwrap()");

    function.line(format!(
        ".json::<{}>()",
        response_aliases.current.variable_alias
    ));

    function
        .line(".await")
        .line(".map_err(|error| error.status().unwrap())");

    match number_depending {
        number if number >= 1 => {
            function.line("})");
        }
        number if number == 0 => {
            function.line("}");
        }
        _ => {}
    }

    function.line(".boxed()").line(".shared();");
}
