use codegen::Scope;

pub fn generate_imports(scope: &mut Scope, request_module_name: String) {
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");
    scope.import("warp", "reply");
    scope.import(format!("super::{}", request_module_name).as_str(), "*");
}
