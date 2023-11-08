use std::{
    collections::BTreeMap,
    fs::{self, create_dir_all},
    path::Path,
};

use codegen::Scope;

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
        self.modules
            .insert("create_filter".to_string(), generate_create_filter());

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

fn generate_create_filter() -> String {
    let mut scope = Scope::new();

    scope.import("warp", "Filter");
    scope.import("futures", "Future");

    scope.import("super::workflow_request_definition", "define_request");
    scope.import("super::workflow_response_definition", "map_response");

    scope
        .new_fn("create_filter")
        .vis("pub")
        .ret(
            "impl Filter<
        Extract = impl warp::Reply,
        Error = warp::Rejection,
        Future = impl Future<Output = Result<impl warp::Reply, warp::Rejection>>,
    > + Clone",
        )
        .line("define_request().and_then(map_response)");

    scope.to_string()
}
