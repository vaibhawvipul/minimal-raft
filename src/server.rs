use std::sync::{atomic, mpsc};
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{Receiver, Sender};
use rand::Rng;
use std::thread;
use std::time::Duration;

pub enum ServerState {
    Follower,
    Candidate,
    Leader,
}
pub struct Server {
    pub id: u64,
    pub current_term: atomic::AtomicU64,
    pub voted_for: Option<u64>,
    pub log: Vec<u8>,
    pub commit_index: u64,
    pub state: ServerState,
    pub election_timeout: u64, // 150-300ms
}

// define Message enum
pub enum Message {
    AppendEntries(AppendEntriesArgs),
    RequestVote(RequestVoteArgs),
}

// define AppendEntriesArgs struct
pub struct AppendEntriesArgs {
    pub term: u64,
    pub leader_id: u64,
    pub prev_log_index: u64,
    pub entries: Vec<u8>,
    pub leader_commit: u64,
}

// define RequestVoteArgs struct
pub struct RequestVoteArgs {
    pub term: AtomicU64,
    pub candidate_id: u64,
    pub last_log_index: u64,
}

// server trait
trait ServerTrait {
    fn init(id: u64) -> Self;

    fn run() -> Self;

    fn run_election_timeout(&mut self, rx: Receiver<Message>) -> self;

    fn send_heartbeat(&mut self, tx: Sender<Message>) -> self;

    fn start_election(&mut self, tx: Sender<Message>) -> self;

    fn send_append_entries(&mut self, tx: Sender<Message>) -> self;

    fn send_request_vote(&mut self, tx: Sender<Message>) -> self;

    fn handle_append_entries(&mut self, tx: Sender<Message>) -> self;

    fn handle_request_vote(&mut self, tx: Sender<Message>) -> self;
}

impl ServerTrait for Server {
    fn init(id: u64) -> Self {
        Server {
            id,
            current_term: AtomicU64::new(0),
            voted_for: None,
            log: vec![],
            commit_index: 0,
            state: ServerState::Follower,
            election_timeout: 0,
        }
    }

    // election time out should be running in background
    fn run_election_timeout(&mut self, rx: Receiver<Message>) -> self {
        // election timeout should be random between 150-300ms
        let election_timeout = rand::thread_rng().gen_range((150, 300));
        self.election_timeout = election_timeout;
        let mut server = self.clone();

        // start a timeout in thread,
        // it get interrupted when receive a message till election_timout else it will start election
        thread::spawn(move || {
            loop {
                match rx.recv_timeout(Duration::from_millis(server.election_timeout)) {
                    Ok(message) => {
                        // reset election timeout after receive message
                        server.election_timeout = rand::thread_rng().gen_range((150, 300));
                        // handle message
                        match message {
                            Message::AppendEntries(args) => {
                                server.handle_append_entries(args);
                            }
                            Message::RequestVote(args) => {
                                server.handle_request_vote(args);
                            }
                        }
                    }
                    Err(_) => {
                        // start election
                        server.start_election();
                    }
                }
            }
        });

        self
    }

    // start election will call a function from leader
    fn start_election(&mut self, tx: Sender<Message>) -> self {
        todo!()
    }

    // send heartbeat will call a function from leader
    fn send_heartbeat(&mut self, tx: Sender<Message>) -> self {
        todo!()
    }

    // send append entries will call a function from leader
    fn send_append_entries(&mut self, tx: Sender<Message>) -> self {
        todo!()
    }

    // send request vote will call a function from leader
    fn send_request_vote(&mut self, tx: Sender<Message>) -> self {
        todo!()
    }

    // handle append entries will call a function from leader
    fn handle_append_entries(&mut self, tx: Sender<Message>) -> self {
        todo!()
    }

    // handle request vote will call a function from leader
    fn handle_request_vote(&mut self, tx: Sender<Message>) -> self {
        todo!()
    }

    // run will call a function from leader
    fn run() -> Self {
        todo!()
    }
}