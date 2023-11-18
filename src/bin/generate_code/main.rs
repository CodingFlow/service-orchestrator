mod generate_create_filter;
mod generate_re_exports;
mod generate_workflows;
pub mod parse_specs;
pub mod traversal;

use generate_re_exports::{ReExports, ReExportsBehavior};
use generate_workflows::generate_workflows;
use parse_specs::{get_operation_specs, SpecType};

fn main() {
    let mut re_exports = ReExports::new();

    let workflow_specs = get_operation_specs(SpecType::Workflow);
    generate_workflows(workflow_specs, &mut re_exports);

    re_exports.generate();
}
