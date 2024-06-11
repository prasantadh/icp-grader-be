#![allow(unused)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod group;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Success,
    Failure,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: Status,
    pub data: Value,
}

impl Response {
    fn new(status: Status, data: Value) -> Self {
        Response { status, data }
    }
}
