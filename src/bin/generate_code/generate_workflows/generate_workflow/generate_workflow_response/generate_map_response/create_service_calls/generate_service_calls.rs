use crate::generate_workflows::input_map::Variable;
use crate::traversal::traverse_nested_node;
use crate::traversal::NestedNode;

use super::build_loopkup_map::ServiceCodeGenerationInfo;
use codegen::Function;
use codegen::Scope;
use std::collections::BTreeMap;
use std::fmt::format;

pub fn generate_service_calls(
    scope: &mut Scope,
    function: &mut Function,
    generation_infos: (
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
) {
    generate_imports(scope);

    generate_futures(function, generation_infos);
}

fn generate_imports(scope: &mut Scope) {
    scope.import("reqwest", "Client");
    scope.import("tokio_stream", "StreamExt");
    scope.import("futures", "FutureExt");
}

fn generate_futures(
    function: &mut Function,
    (generation_infos, ordered_generation_infos): (
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
) {
    function.line("let client = Client::new();");

    for ((service_name, operation_id), service_code_generation_info) in
        ordered_generation_infos.iter().rev()
    {
        generate_future(service_code_generation_info, &generation_infos, function);
    }
}

fn generate_future(
    service_code_generation_info: &ServiceCodeGenerationInfo,
    generation_infos: &BTreeMap<(String, String), ServiceCodeGenerationInfo>,
    mut function: &mut Function,
) {
    let ServiceCodeGenerationInfo {
        future_variable_name,
        response_struct_name,
        response_aliases,
        depending_service_names,
        request,
    } = service_code_generation_info;

    let number_depending = depending_service_names.len();

    match number_depending {
        number if number > 1 => {}
        number if number == 1 => {
            let depending_service_name = depending_service_names.first().unwrap();
            let depending_service_code_generation_info =
                generation_infos.get(depending_service_name).unwrap();
            let depending_service_struct_name =
                &depending_service_code_generation_info.response_struct_name;

            function
                .line(format!(
                    "let {} = {}",
                    future_variable_name,
                    depending_service_code_generation_info.future_variable_name,
                ))
                .line(".clone()")
                .line(".then(|result| async {");

            function.line(format!("let {} {{", depending_service_struct_name));

            traverse_nested_node(
                depending_service_code_generation_info.response_aliases.clone(),
                |parent_node: NestedNode<Option<Variable>>,
                 (function, service_struct_name): &mut (&mut &mut Function, &String)| {
                    // if parent_node.children.is_some() {
                    //     function.line("},");
                    // }

                    if let Some(node) = parent_node.current {
                        match parent_node.children.is_some() {
                            true => {
                                function.line(format!(
                                    "{}: {} {{",
                                    node.original_name, service_struct_name
                                ));
                            }
                            false => {
                                function.line(format!(
                                    "{}: {},",
                                    node.original_name, node.alias
                                ));
                            }
                        };
                    }
                },
                |child_node, _, (function, service_struct_name)| {},
                &mut (&mut function, depending_service_struct_name),
            );

            function.line("} = result.unwrap();");
        }
        number if number == 0 => {
            function.line(format!("let {} = async {{", future_variable_name));
        }
        _ => {}
    };

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
        r#".{}(format!("http://localhost:3001/{}", {}))"#,
        request.method.to_string().to_lowercase(),
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

    function.line(format!(".json::<{}>()", response_struct_name));

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
