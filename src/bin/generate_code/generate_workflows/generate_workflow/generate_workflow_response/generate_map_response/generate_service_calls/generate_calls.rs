mod generate_futures;
mod generate_imports;
mod generate_response_handling;
mod generate_streams;

use crate::generate_workflows::generate_workflow::build_service_call_view_data::build_service_operation_lookup_map::ServiceCodeGenerationInfo;
use crate::generate_workflows::generate_workflow::build_service_call_view_data::build_workflow_response_lookup_map::WorkflowResponseCodeGenerationInfo;
use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_response_variables::ServiceResponseAlias;
use crate::generate_workflows::generate_workflow::build_service_call_view_data::generate_response_variables::generate_response_variables;
use crate::generate_workflows::generate_workflow::variables::VariableAliases;
use crate::traversal::NestedNode;
use codegen::Function;
use codegen::Scope;
use generate_futures::generate_futures;
use generate_imports::generate_imports;
use generate_response_handling::generate_response_handling;
use generate_streams::generate_streams;
use std::collections::BTreeMap;

pub fn generate_calls(
    scope: &mut Scope,
    function: &mut Function,
    generation_infos: (
        BTreeMap<(String, String), ServiceCodeGenerationInfo>,
        Vec<((String, String), ServiceCodeGenerationInfo)>,
    ),
    workflow_response_code_generation_info: WorkflowResponseCodeGenerationInfo,
    variable_aliases: &mut VariableAliases,
) {
    generate_imports(scope);

    generate_futures(function, &generation_infos, variable_aliases);

    generate_streams(function, generation_infos.1.to_vec());

    generate_response_handling(function, workflow_response_code_generation_info);
}

pub fn generate_response_variables_assigned(
    function: &mut Function,
    response_aliases: &NestedNode<ServiceResponseAlias>,
) {
    function.line("let ");

    generate_response_variables(function, response_aliases);
}
