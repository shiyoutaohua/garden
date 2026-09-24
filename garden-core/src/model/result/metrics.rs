use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppMetrics {
    pub uptime: u64,
    pub start_ts: u64,
    pub start_iso: String,
}
