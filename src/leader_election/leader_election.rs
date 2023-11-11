use tokio::sync::mpsc;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::{Server, Message};

pub async fn start_election(server: Arc<Mutex<Server>>, tx: &mpsc::Sender<Message>) {
    let mut server = server.lock().unwrap();

    // check if server is not a leader
    if server.state != ServerState::Leader {
        return;
    }

    server.state = ServerState::Candidate;
    server.current_term += 1;
    server.voted_for = Some(server.id);

    // Send RequestVote to all other servers
    let request_vote_args = RequestVoteArgs {
        term: server.current_term,
        candidate_id: server.id,
        last_log_index: server.log.len() as u64,
        last_log_term: server.current_term,
    };
    for _ in 1..=3 {
        tx.send(Message::RequestVote(request_vote_args.clone())).await.unwrap();
    }
}
