use super::SpecType;

pub fn get_directory_path(spec_type: SpecType) -> String {
    match spec_type {
        SpecType::Workflow => "./src/workflow_specs/".to_string(),
        SpecType::Service => "./src/service_specs/".to_string(),
    }
}
