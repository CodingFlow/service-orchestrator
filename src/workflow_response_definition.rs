mod workflow_response;

use std::collections::HashMap;

use warp::reply::{self, Json};

use self::workflow_response::WorkflowResponse;

pub fn map_response(name: String, parameters: HashMap<String, String>) -> Json {
    match parameters.get("key") {
        Some(key) => reply::json(&WorkflowResponse {
            name,
            key: key.to_string(),
        }),
        None => reply::json(&WorkflowResponse {
            name: "blah".to_string(),
            key: "no key!".to_string(),
        }),
    }
}
