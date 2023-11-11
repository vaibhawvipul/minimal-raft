use std::sync::atomic;

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
    pub last_applied: u64,
    pub state: ServerState,
}

// server trait
trait ServerTrait {
    fn run(rx: mpsc::Receiver<Message>) -> Self;

    fn start_election(&mut self, tx: &mpsc::Sender<Message>) -> self;

    fn send_append_entries(&mut self, tx: &mpsc::Sender<Message>) -> self;

    fn send_request_vote(&mut self, tx: &mpsc::Sender<Message>) -> self;

    fn handle_append_entries(&mut self, tx: &mpsc::Sender<Message>) -> self;

    fn handle_request_vote(&mut self, tx: &mpsc::Sender<Message>) -> self;
}

impl ServerTrait for Server {
    fn run(rx: mpsc::Receiver<Message>) -> Self {
        let mut server = Server {
            id: 0,
            current_term: 0,
            voted_for: None,
            log: vec![],
            commit_index: 0,
            last_applied: 0,
            state: ServerState::Follower,
        };
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                match message {
                    Message::AppendEntries(args) => {
                        // Your logic for handling AppendEntries goes here
                    }
                    Message::RequestVote(args) => {
                        // Your logic for handling RequestVote goes here
                    }
                }
            }
        });
        server
    }

    fn start_election(&mut self, tx: &mpsc::Sender<Message>) {
    }
}