mod test;
mod workflow_request_definition;
mod workflow_response_definition;

use warp::Filter;
use workflow_request_definition::define_request;
use workflow_response_definition::map_response;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let filter = create_filter();

    set_up_server(filter).await;
}

fn create_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    define_request().map(map_response)
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
