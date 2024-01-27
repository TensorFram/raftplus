#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub term: u64,
    pub command: String,
}

impl LogEntry {
    pub fn new(term: u64, command: String) -> LogEntry {
        LogEntry {
            term,
            command,
        }
    }
}