use codegen::{Function, Scope};

use crate::generate_re_exports::{ReExports, ReExportsBehavior};

pub struct WorkflowDefinitionNames {
    pub request_name: String,
    pub response_name: String,
}

pub fn generate_create_filter(
    workflow_definition_names: Vec<WorkflowDefinitionNames>,
    re_exports: &mut ReExports,
) {
    let code = generate_code(workflow_definition_names);

    re_exports.add("create_filter".to_string(), code);
}

fn generate_code(workflow_definition_names: Vec<WorkflowDefinitionNames>) -> String {
    let mut scope = Scope::new();

    let method_names = generate_imports(&mut scope, workflow_definition_names);

    let function = generate_function_signature(&mut scope);

    generate_function_body(function, method_names);

    scope.to_string()
}

fn generate_imports(
    scope: &mut Scope,
    workflow_definition_names: Vec<WorkflowDefinitionNames>,
) -> Vec<(String, String)> {
    scope.import("warp", "Filter");
    scope.import("futures", "Future");

    let import_parts: Vec<((String, String), (String, String))> = workflow_definition_names
        .iter()
        .map(|names| -> ((String, String), (String, String)) {
            let WorkflowDefinitionNames {
                request_name,
                response_name,
            } = names;

            (
                (
                    format!("super::{}", request_name),
                    format!("define_request_{}", request_name),
                ),
                (
                    format!("super::{}", response_name),
                    format!("map_response_{}", response_name),
                ),
            )
        })
        .collect();

    for ((request_path, request_type), (response_path, response_type)) in &import_parts {
        scope.import(
            &request_path,
            &format!("define_request as {}", request_type),
        );
        scope.import(
            &response_path,
            &format!("map_response as {}", response_type),
        );
    }

    import_parts
        .iter()
        .map(
            |((_, request_type), (_, response_type))| -> (String, String) {
                (request_type.to_string(), response_type.to_string())
            },
        )
        .collect()
}

fn generate_function_signature(scope: &mut Scope) -> &mut Function {
    scope.new_fn("create_filter").vis("pub").ret(
        "impl Filter<
        Extract = impl warp::Reply,
        Error = warp::Rejection,
        Future = impl Future<Output = Result<impl warp::Reply, warp::Rejection>>,
    > + Clone",
    )
}

fn generate_function_body(function: &mut Function, method_names: Vec<(String, String)>) {
    let (first, rest) = method_names.split_first().unwrap();

    let (request_method_name, response_method_name) = first;
    function.line(format!(
        "{}().and_then({})",
        request_method_name, response_method_name
    ));

    for (request_method_name, response_method_name) in rest {
        function.line(format!(
            ".or({}().and_then({}))",
            request_method_name, response_method_name
        ));
    }
}
