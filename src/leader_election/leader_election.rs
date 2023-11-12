use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use rand::Rng;
use crate::server::{RequestVoteArgs, Server, ServerMetaData, ServerState};
use crate::server::Message;

pub async fn start_election(server: Arc<Mutex<ServerMetaData>>, tx: &mpsc::Sender<Message>) {
    let mut server = server.lock().unwrap();

    // check if server is not a leader
    if server.state != ServerState::Leader {
        return;
    }

    server.state = ServerState::Candidate;
    server.current_term.fetch_add(1, Ordering::Relaxed);

    // vote for self
    server.voted_for = Some(server.id);

    // request for votes from other servers
    let request_vote_args = RequestVoteArgs {
        term: server.current_term.clone(),
        candidate_id: server.id,
        last_log_index: server.log.len() as u64,
    };

    // send request vote to all other servers
    for _ in 1..=3 {
        tx.send(Message::RequestVote(request_vote_args.clone())).await.unwrap();
    }

    // reset election timeout
    server.election_timeout = rand::thread_rng().gen_range((150, 300));

}
