use codegen::Scope;

pub fn generate_imports(scope: &mut Scope, query_struct_name: &str) {
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");
    scope.import("warp::reply", "self");
    scope.import("warp::reply", "Json");
    scope.import("super::workflow_request_definition", query_struct_name);
}
