enum SeverState {
    Candidate,
    Follower,
    Leader,
}

struct Server {
    id: u64,
    state: ServerState,
    current_term: u64,
    voted_for: Option<u64>,
    log: Vec<LogEntry>,
    commit_index: u64,
    next_index: Vec<u64>,
    match_index: Vec<u64>,
}

impl Server {
    fn new(id: u64) -> Server {
        // check if storage is stable
        Server {
            id,
            state: ServerState::Follower,
            current_term: 0,
            voted_for: None,
            log: vec![],
            commit_index: 0,
            last_applied: 0,
            next_index: vec![],
            match_index: vec![],
        }
    }
    fn run(&mut self) {
        loop {
            match self.state {
                ServerState::Follower => {
                    self.run_as_follower();
                }
                ServerState::Candidate => {
                    self.run_as_candidate();
                }
                ServerState::Leader => {
                    self.run_as_leader();
                }
            }
        }
    }
    fn run_as_follower(&mut self) {
        // check timeout
        

    }
    fn run_as_candidate(&mut self) {
        // TODO
    }
    fn run_as_leader(&mut self) {
        // TODO
    }
    fn restart(&mut self, i: u64) {
        self.state = ServerState::Follower;
        self.current_term = i;
        self.voted_for = None;
        self.log = vec![];
        self.commit_index = 0;
        self.last_applied = 0;
        self.next_index = vec![];
        self.match_index = vec![];
    }
    fn timeout(&mut self, i: u64) {
        if self.state == ServerState::Follower || self.state == ServerState::Candidate {
            self.state = ServerState::Candidate;
            self.current_term += 1;
            self.voted_for = Some(self.id);
        }
    }
}