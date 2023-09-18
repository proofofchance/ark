use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerDataResponse<T> {
    pub data: T,
    pub data_type: &'static str,
}

// TODO: Make a macro to build different data types
