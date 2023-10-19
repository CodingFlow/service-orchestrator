use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorkflowResponse {
    pub name: String,
    pub key: String,
}
