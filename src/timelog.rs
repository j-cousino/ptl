use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub email: String,
    pub start: chrono::DateTime<Local>,
    pub stop: Option<chrono::DateTime<Local>>,
}

impl LogEntry {
    pub fn new(e: &str) -> LogEntry {
        LogEntry {
            email: e.to_string(),
            start: chrono::Local::now(),
            stop: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timelog {
    pub entries: Vec<LogEntry>,
}

impl Timelog {
    pub fn new(e: &str) -> Timelog {
        Timelog { entries: vec![LogEntry::new(e)], }
    }
}