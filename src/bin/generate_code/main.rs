mod generate_create_filter;
mod generate_re_exports;
mod generate_workflows;
mod get_service_urls;

pub mod parse_specs;
pub mod traversal;

use generate_re_exports::{ReExports, ReExportsBehavior};
use generate_workflows::generate_workflows;
use get_service_urls::get_service_urls;
use parse_specs::{get_operation_specs, SpecType};

fn main() {
    let mut re_exports = ReExports::new();

    let workflow_specs = get_operation_specs(SpecType::Workflow);
    let service_operation_specs = get_operation_specs(SpecType::Service);
    let service_urls = get_service_urls();

    generate_workflows(
        workflow_specs,
        service_operation_specs,
        service_urls,
        &mut re_exports,
    );

    re_exports.generate();
}
