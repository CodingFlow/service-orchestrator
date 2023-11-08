mod extract_request_values_from_spec;
mod generate_create_filter;
mod generate_re_exports;
mod generate_workflows;
mod parse_workflow_specs;
pub mod spec_parsing;
pub mod traversal;

use generate_re_exports::{ReExports, ReExportsBehavior};
use generate_workflows::generate_workflows;
use parse_workflow_specs::{parse_workflow_specs, SpecInfo};

fn main() {
    let mut re_exports = ReExports::new();

    let workflow_spec_infos = parse_workflow_specs();
    generate_workflows(workflow_spec_infos, &mut re_exports);

    re_exports.generate();
}
