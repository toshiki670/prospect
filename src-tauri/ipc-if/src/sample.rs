use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = "sample.ts")]
pub struct SampleQuery {
    pub query: Option<String>,
}

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = "sample.ts")]
pub struct Sample {
    pub id: u32,
    pub title: String,
    pub body: String,
}
