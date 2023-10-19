use std::collections::HashMap;

use oas3::Spec;
use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, Filter};

const PATH_PARTS: [&str; 2] = ["hello", "bye"];

#[derive(Serialize, Deserialize)]
struct WorkflowRequestConfig {}

pub fn define_request(
) -> impl Filter<Extract = (String, HashMap<String, String>), Error = warp::Rejection> + Clone {
    let spec = parse_config();

    let http_method = define_method(spec);

    let with_paths = define_paths(http_method);

    let with_query = define_query(with_paths);

    with_query
}

fn define_method(spec: Spec) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    let path_item = match spec.paths.first_key_value() {
        Some(item) => item.1,
        None => panic!("Endpoint method missing"),
    };

    if path_item.get.is_some() {
        warp::get()
    } else {
        panic!("could not get endpoint method");
    }
}

fn define_paths(
    http_method: impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy,
) -> impl Filter<Extract = (String,), Error = warp::reject::Rejection> + Copy {
    http_method
        .and(warp::path(PATH_PARTS[0]))
        .and(warp::path::param::<String>())
        .and(warp::path(PATH_PARTS[1]))
}

fn define_query(
    with_paths: impl Filter<Extract = (String,), Error = Rejection> + Copy,
) -> impl Filter<Extract = (String, HashMap<String, String>), Error = Rejection> + Copy {
    with_paths.and(warp::query::<HashMap<String, String>>())
}

fn parse_config() -> oas3::Spec {
    let spec = match oas3::from_path("./src/workflow_request.yaml") {
        Ok(spec) => spec,
        Err(_) => panic!("unable to read open API spec file"),
    };

    spec
}
