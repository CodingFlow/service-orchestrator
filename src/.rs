use oas3::Spec;
use warp::reject::Rejection;
use warp::Filter;

fn define_method(spec: Spec) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    let path_item = match spec.paths.first_key_value() {
        Some(item) => item.1,
        None => panic!("Endpoint method missing")
    };
    warp::get()
}