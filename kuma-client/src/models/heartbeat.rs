//! Models related to Uptime Kuma heartbeat data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single heartbeat record representing one monitor check result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Heartbeat {
    /// The monitor ID this heartbeat belongs to
    pub monitor_id: Option<i32>,

    /// 0=DOWN, 1=UP, 2=PENDING, 3=MAINTENANCE
    pub status: Option<i32>,

    /// UTC timestamp of the check
    pub time: Option<String>,

    /// Status message or error details
    pub msg: Option<String>,

    /// Response time in milliseconds
    pub ping: Option<i32>,

    /// Whether this status changed (0/1 from DB)
    pub important: Option<i32>,

    /// Duration since last heartbeat in seconds
    pub duration: Option<i32>,
}

/// Map of monitor ID to its recent heartbeats
pub type HeartbeatList = HashMap<String, Vec<Heartbeat>>;
