mod extract_request_values_from_spec;
mod generate_create_filter;
mod generate_re_exports;
mod generate_workflow;
pub mod spec_parsing;
pub mod traversal;

use std::{fs::read_dir, path::Path};

use generate_create_filter::generate_create_filter;
use generate_re_exports::{ReExports, ReExportsBehavior};
use generate_workflow::generate_workflow;
use oas3::Spec;

fn main() {
    let mut re_exports = ReExports::new();

    let workflow_spec_infos = parse_workflow_specs();
    generate_workflows(workflow_spec_infos, &mut re_exports);

    re_exports.generate();
}

pub struct SpecInfo {
    name: String,
    spec: Spec,
}

fn parse_workflow_specs() -> Vec<SpecInfo> {
    let files = match read_dir(Path::new("./src/workflow_specs/")) {
        Ok(files) => files,
        Err(_) => panic!("Unable to read workflow open api spec files!"),
    };

    files
        .map(|dir_entry| -> SpecInfo {
            let dir_entry = &dir_entry.unwrap();

            SpecInfo {
                name: dir_entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                spec: parse_config(dir_entry.path().as_path()),
            }
        })
        .collect()
}

fn parse_config(path: &Path) -> oas3::Spec {
    match oas3::from_path(path) {
        Ok(spec) => spec,
        Err(_) => panic!("unable to read open API spec file"),
    }
}

fn generate_workflows(workflow_spec_infos: Vec<SpecInfo>, re_exports: &mut ReExports) {
    let mut workflow_definition_names = vec![];

    for spec_info in workflow_spec_infos {
        for (path_string, path_item) in &spec_info.spec.paths {
            for (method, operation) in path_item.methods() {
                let names = generate_workflow(
                    &path_item,
                    operation,
                    &spec_info,
                    method,
                    &path_string,
                    re_exports,
                );

                workflow_definition_names.push(names);
            }
        }
    }

    generate_create_filter(workflow_definition_names, re_exports);
}
