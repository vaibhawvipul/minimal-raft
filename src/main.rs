mod leader_election;
mod log_replication;
mod safety;
mod server;

use leader_election::leader_election;
use log_replication::{log_replication, append_entries_args};
use safety::apply_safety_rules;

fn start_leader(server: Arc<Mutex<Server>>, tx: &mpsc::Sender<Message>) {
    let mut server = server.lock().unwrap();

    // Your leader logic goes here

    // For illustration purposes, let's assume a simple leader that sends
    // AppendEntries messages to other servers.

    // Send AppendEntries to all other servers
    let append_entries_args = append_entries_args {
        term: server.current_term,
        leader_id: server.id,
        prev_log_index: server.log.len() as u64,
        prev_log_term: server.current_term,
        entries: vec![],
        leader_commit: server.commit_index,
    };
    for _ in 1..=3 {
        tx.send(Message::AppendEntries(append_entries_args.clone())).await.unwrap();
    }
}

fn start_worker(server: Arc<Mutex<Server>>, tx: &mpsc::Sender<Message>) {
    let mut server = server.lock().unwrap();

    // Your worker logic goes here

    // For illustration purposes, let's assume a simple worker that sends
    // AppendEntries messages to other servers.

    // Send AppendEntries to all other servers
    let append_entries_args = append_entries_args {
        term: server.current_term,
        leader_id: server.id,
        prev_log_index: server.log.len() as u64,
        prev_log_term: server.current_term,
        entries: vec![],
        leader_commit: server.commit_index,
    };
    for _ in 1..=3 {
        tx.send(Message::AppendEntries(append_entries_args.clone())).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // ... initialization code ...

    let (tx, rx) = mpsc::channel::<Message>(1024);

    let server = tokio::spawn(Server::run(rx));

    // Example: Send a message to the server
    tx.send(Message::AppendEntries(/*...*/)).await.unwrap();

    // ... more code ...

    server.await.unwrap();
}

fn main() {}