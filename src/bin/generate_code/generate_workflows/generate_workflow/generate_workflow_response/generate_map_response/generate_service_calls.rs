mod generate_calls;
mod generate_service_response_structs;
mod generate_stream_enum;

use crate::generate_workflows::generate_workflow::{
    build_service_call_view_data::ServiceCallGenerationInfo, variables::VariableAliases,
};
use codegen::{Function, Scope};

use generate_calls::generate_calls;
use generate_service_response_structs::generate_service_response_structs;
use generate_stream_enum::generate_stream_enum;

pub fn generate_service_calls(
    function: &mut Function,
    scope: &mut Scope,
    variable_aliases: &mut VariableAliases,
    service_call_view_data: ServiceCallGenerationInfo,
) {
    let ServiceCallGenerationInfo {
        service_calls: service_response_infos,
        workflow_service_response: workflow_response_info,
    } = service_call_view_data;

    generate_service_response_structs(scope, service_response_infos.1.to_vec());

    generate_stream_enum(scope, service_response_infos.1.to_vec());

    generate_calls(
        scope,
        function,
        service_response_infos,
        workflow_response_info,
        variable_aliases,
    );
}
