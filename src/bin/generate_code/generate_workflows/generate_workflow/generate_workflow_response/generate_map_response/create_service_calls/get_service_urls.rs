use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use url::Url;

pub fn get_service_urls() -> BTreeMap<String, Url> {
    let file = match fs::File::open("./src/service_config.yaml") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read service configuration file."),
    };

    let config: Value = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse service configuration file."),
    };

    config
        .as_object()
        .unwrap()
        .into_iter()
        .map(|(service_name, value)| {
            (
                service_name,
                value.as_object().unwrap().get("service_url").unwrap(),
            )
        })
        .map(|(service_name, value)| {
            let url = value.as_str().unwrap();
            (service_name.to_string(), Url::parse(url).unwrap())
        })
        .collect()
}
