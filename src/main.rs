mod generated_re_exports;

use generated_re_exports::create_filter::create_filter;
use warp::Filter;

#[tokio::main]
async fn main() {
    let filter = create_filter();

    set_up_server(filter).await;
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
