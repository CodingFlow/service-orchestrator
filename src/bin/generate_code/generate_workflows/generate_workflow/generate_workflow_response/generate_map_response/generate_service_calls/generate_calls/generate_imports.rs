use codegen::Scope;

pub fn generate_imports(scope: &mut Scope) {
    scope.import("reqwest", "Client");
    scope.import("tokio_stream", "StreamExt");
    scope.import("futures", "FutureExt");
    scope.import("http", "StatusCode");
}
