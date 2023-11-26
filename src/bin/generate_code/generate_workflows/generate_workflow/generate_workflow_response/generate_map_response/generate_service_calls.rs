mod generate_calls;
mod generate_service_response_structs;
mod generate_stream_enum;

use crate::generate_workflows::generate_workflow::{
    build_service_call_view_data::{
        build_service_operation_lookup_map::ServiceCodeGenerationInfo, ServiceCallGenerationInfo,
    },
    generate_structs::generate_structs,
    variables::VariableAliases,
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

    generate_service_request_body_structs(scope, service_response_infos.1.to_vec());

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

fn generate_service_request_body_structs(
    scope: &mut Scope,
    generation_infos: Vec<((String, String), ServiceCodeGenerationInfo)>,
) {
    for (_, info) in generation_infos.to_vec() {
        if let Some(body) = info.request.body {
            generate_structs(body);
        }
    }

    let structs_iter = generation_infos
        .iter()
        .filter(|(_, info)| info.request.body.is_some())
        .flat_map(|(_, info)| generate_structs(info.request.body.clone().unwrap()));

    for new_struct in structs_iter {
        scope.push_struct(new_struct);
    }
}
