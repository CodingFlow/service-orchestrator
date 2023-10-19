use std::collections::HashMap;

use warp::{reject::Rejection, Filter};

const PATH_PARTS: [&str; 2] = ["hello", "bye"];

pub fn define_request(
) -> impl Filter<Extract = (String, HashMap<String, String>), Error = warp::Rejection> + Clone {
    let http_method = define_method();

    define_paths(http_method)
        .and(warp::path::param::<String>())
        .and(warp::query::<HashMap<String, String>>())
}

fn define_method() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::get()
}

fn define_paths(
    http_method: impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy,
) -> impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy {
    http_method
        .and(warp::path(PATH_PARTS[0]))
        .and(warp::path(PATH_PARTS[1]))
}
