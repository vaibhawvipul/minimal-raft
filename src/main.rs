mod leader_election;
mod log_replication;
mod safety;

use leader_election::leader_election;
use log_replication::log_replication;
use safety::apply_safety_rules;

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