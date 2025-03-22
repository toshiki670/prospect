use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = "import.ts")]
pub struct TokyoStockExchangeImportQuery {
    pub japanese_file: Option<String>,
    pub english_file: Option<String>,
}
