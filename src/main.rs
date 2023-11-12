mod leader_election;
mod log_replication;
mod safety;
mod server;

use server::{Server};
use leader_election::leader_election;
use log_replication::{log_replication, append_entries_args};
use crate::server::ServerMetaData;

fn main() {
    // ask how many servers are in the cluster, default is 3
    let mut num_servers = 3;
    let mut hostname = String::from("0.0.0.0");
    let mut port_vec = vec![8080, 8081, 8082];

    // todo: add command line arguments to change the number of servers, hostname, and ports

    // create a vector of servers
    let mut cluster: Vec<ServerMetaData> = vec![];

    for i in 1..num_servers {
        cluster.push(Server::init(i, hostname.clone(), port_vec[i]));
    }

    // start the servers
    for server in cluster.iter_mut() {
        server.run();
    }

    // send request to the cluster
}
