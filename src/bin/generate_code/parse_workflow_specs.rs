use std::{fs::read_dir, path::Path};

use oas3::Spec;

pub struct SpecInfo {
    pub name: String,
    pub spec: Spec,
}

pub fn parse_workflow_specs() -> Vec<SpecInfo> {
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
