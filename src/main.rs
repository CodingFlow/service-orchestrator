mod generated_re_exports;

use std::future::Future;

use generated_re_exports::{
    workflow_request_definition::define_request, workflow_response_definition::map_response,
};

use warp::Filter;

#[tokio::main]
async fn main() {
    let filter = create_filter();

    set_up_server(filter.clone().or(filter)).await;
}

fn create_filter() -> impl Filter<
    Extract = impl warp::Reply,
    Error = warp::Rejection,
    Future = impl Future<Output = Result<impl warp::Reply, warp::Rejection>>,
> + Clone {
    define_request()
        .and_then(map_response)
        .or(define_request().and_then(map_response))
}

async fn set_up_server(
    filter: impl Filter<Extract = impl warp::Reply, Error = warp::Rejection>
        + Clone
        + Sync
        + Send
        + 'static,
) {
    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}
