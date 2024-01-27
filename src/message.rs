#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    // for follower
    AppendEntriesRequest {
        term: u64,
        leader_id: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: u64,
    },
    AppendEntriesResponse {
        term: u64,
        success: bool,
        match_index: u64,
    },
    // for candidate
    RequestVoteRequest {
        term: u64,
        candidate_id: u64,
        last_log_index: u64,
        last_log_term: u64,
    },
    RequestVoteResponse {
        term: u64,
        vote_granted: bool,
    },
    // for client
    ClientRequest {
        command: String,
    },
    ClientResponse {
        success: bool,
        command: String,
    },
}
