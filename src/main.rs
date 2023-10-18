use std::collections::HashMap;

use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
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

fn create_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        // .map(|name| format!("Hello, {}!!", name))
        .and(warp::query::<HashMap<String, String>>())
        .map(
            |name: String, p: HashMap<String, String>| match p.get("key") {
                Some(key) => Response::builder().body(format!("key = {}. Hello {}!!!", key, name)),
                None => Response::builder().body(String::from("No \"key\" param in query.")),
            },
        )
}
