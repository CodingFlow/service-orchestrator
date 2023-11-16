mod generate_futures;

use super::build_loopkup_map::ServiceCodeGenerationInfo;
use super::variables::VariableAliases;
use codegen::Function;
use codegen::Scope;
use generate_futures::generate_futures;
use std::collections::BTreeMap;

pub fn generate_service_calls(
    scope: &mut Scope,
    function: &mut Function,
    generation_infos: (
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
    variable_aliases: VariableAliases,
) {
    generate_imports(scope);

    generate_futures(function, generation_infos, variable_aliases);
}

fn generate_imports(scope: &mut Scope) {
    scope.import("reqwest", "Client");
    scope.import("tokio_stream", "StreamExt");
    scope.import("futures", "FutureExt");
}
