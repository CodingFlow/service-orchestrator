use std::{
    collections::BTreeMap,
    fs::{self, create_dir_all},
    path::Path,
};

pub struct ReExports {
    modules: BTreeMap<String, String>,
}

impl ReExports {
    pub fn new() -> ReExports {
        ReExports {
            modules: BTreeMap::new(),
        }
    }
}

pub trait ReExportsBehavior {
    fn add(&mut self, module_name: String, code: String);
    fn generate(&mut self);
}

impl ReExportsBehavior for ReExports {
    fn generate(&mut self) {
        for (module_name, code) in &self.modules {
            let file_name = format!("./src/generated_re_exports/{}.rs", module_name);
            let path = Path::new(&file_name);

            let _ = create_dir_all(path.parent().unwrap());
            let _ = fs::write(file_name, code);
        }

        let formatted_module_names: Vec<String> = self
            .modules
            .keys()
            .map(|module_name| -> String { format!("pub mod {};", module_name.to_string()) })
            .collect();

        let _ = fs::write(
            Path::new("./src/generated_re_exports.rs"),
            formatted_module_names.join("\n"),
        );
    }

    fn add(&mut self, module_name: String, code: String) {
        self.modules.insert(module_name, code);
    }
}
