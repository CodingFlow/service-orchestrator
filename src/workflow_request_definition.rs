use std::collections::HashMap;
use warp::reject::Rejection;
use warp::Filter;

pub fn define_request(
) -> impl Filter<Extract = (f32, HashMap<String, String>), Error = warp::Rejection> + Clone {
    let http_method = define_method();
    let with_paths = define_paths(http_method);
    let with_query = define_query(with_paths);
    with_query
}

fn define_method() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::get()
}

fn define_paths(
    http_method: impl Filter<Extract = (), Error = warp::reject::Rejection> + Copy,
) -> impl Filter<Extract = (f32,), Error = warp::reject::Rejection> + Copy {
    http_method
        .and(warp::path("cat"))
        .and(warp::path::param::<f32>())
        .and(warp::path::end())
}

fn define_query(
    with_paths: impl Filter<Extract = (f32,), Error = Rejection> + Copy,
) -> impl Filter<Extract = (f32, HashMap<String, String>), Error = Rejection> + Copy {
    with_paths.and(warp::query::<HashMap<String, String>>())
}
