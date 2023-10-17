use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let filter = create_filter();

    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}

fn create_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    return warp::path!("hello" / String).map(|name| format!("Hello, {}!!", name));
}
