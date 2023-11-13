mod leader_election;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use rand::Rng;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone, Debug)]
pub enum ServerState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug)]
pub struct ServerMetaData {
    pub id: u64,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<u8>,
    pub commit_index: u64,
    pub state: ServerState,
    pub election_timeout: u64, // 150-300ms
    pub hostname: String,
    pub port: u64,
    pub listener: TcpListener,
}

// define Message enum
#[derive(Clone, Debug)]
pub enum Message {
    AppendEntries(AppendEntriesArgs),
    RequestVote(RequestVoteArgs),
}

// define AppendEntriesArgs
#[derive(Clone, Debug)]
pub struct AppendEntriesArgs {
    pub term: u64,
    pub leader_id: u64,
    pub prev_log_index: u64,
    pub entries: Vec<u8>,
    pub leader_commit: u64,
}

// define RequestVoteArgs struct
#[derive(Clone, Debug)]
pub struct RequestVoteArgs {
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
}

// server trait
pub trait Server {
    fn init(id: u64, hostname: String, port: u64) -> Self;

    fn run(&mut self, cluster: Vec<ServerMetaData>) -> Self;

    fn run_election_timeout(&mut self, cluster: Vec<ServerMetaData>) -> Self;

    fn send_heartbeat(&mut self) -> Self;

    fn start_leader_election(&mut self, cluster: Vec<ServerMetaData>) -> Self;

    fn send_append_entries<T>(&mut self, x: Box<T>) -> Self;

    fn send_request_vote(&mut self, request_vote_args: RequestVoteArgs) -> Self;

    fn handle_append_entries(&mut self, cluster: Vec<ServerMetaData>);

    fn handle_request_vote(&mut self) -> Self;
}

impl Server for ServerMetaData {
    fn init(id: u64, hostname: String, port: u64) -> Self {
        ServerMetaData {
            id,
            current_term: 0,
            voted_for: None,
            log: vec![],
            commit_index: 0,
            state: ServerState::Follower,
            election_timeout: 0,
            hostname: hostname.clone(),
            port,
            listener: TcpListener::bind(format!("{}:{}", hostname.clone(), port)).unwrap(),
        }
    }

    // run will call a function from leader
    fn run(&mut self, cluster: Vec<ServerMetaData>) -> Self {

        self.run_election_timeout(cluster);

        if self.state == ServerState::Leader {
            // run leader function
            self.handle_append_entries(vec![]);

        } else if self.state == ServerState::Candidate {
            // run candidate function
            self.handle_request_vote();

        } else {
            println!("Follower... Manual intervention required!");
        }
        todo!()
    }

    fn run_election_timeout(&mut self, cluster: Vec<ServerMetaData>) -> Self {

        // check if you are leader, if so, don't run election timeout
        if self.state == ServerState::Leader {
            return Self {
                id: self.id,
                current_term: self.current_term,
                voted_for: self.voted_for,
                log: self.log.clone(),
                commit_index: self.commit_index,
                state: self.state.clone(),
                election_timeout: self.election_timeout,
                hostname: self.hostname.clone(),
                port: self.port,
                listener: self.listener.try_clone().unwrap(),
            };
        }

        // election timeout should be random between 150-300ms
        let election_timeout = rand::thread_rng().gen_range(150..300);
        self.election_timeout = election_timeout;

        // spawn a thread to listen to server.port, tcp
        thread::spawn(move || {
            for stream in self.listener.incoming() {
                let stream = stream.unwrap();

                // if you receive a message, reset election timeout, else, start election
                if stream.bytes().next().is_some() {
                    self.election_timeout = rand::thread_rng().gen_range(150..300);
                } else {
                    self.start_leader_election(cluster.to_owned());
                }

            }
        });

        Self {
            id: self.id,
            current_term: self.current_term,
            voted_for: self.voted_for,
            log: self.log.clone(),
            commit_index: self.commit_index,
            state: self.state.clone(),
            election_timeout: self.election_timeout,
            hostname: self.hostname.clone(),
            port: self.port,
            listener: self.listener.try_clone().unwrap(),
        }
    }

    fn send_heartbeat(&mut self) -> Self {
        todo!()
    }

    fn start_leader_election(&mut self, mut cluster: Vec<ServerMetaData>) -> Self {

        if self.state != ServerState::Leader {
            return Self {
                id: self.id,
                current_term: self.current_term,
                voted_for: self.voted_for,
                log: self.log.clone(),
                commit_index: self.commit_index,
                state: self.state.clone(),
                election_timeout: self.election_timeout,
                hostname: self.hostname.clone(),
                port: self.port,
                listener: self.listener.try_clone().unwrap(),
            };
        }

        // change state to candidate
        self.state = ServerState::Candidate;

        // increment current term
        self.current_term += 1;

        // vote for self
        self.voted_for = Some(self.id);

        // request for votes from other servers
        let request_vote_args = RequestVoteArgs {
            term: self.current_term.clone(),
            candidate_id: self.id,
            last_log_index: self.log.len() as u64,
        };

        // send request vote to all other servers in cluster
        for server in cluster.iter_mut() {
            if server.id == server.id {
                continue;
            }

            server.send_request_vote(request_vote_args.clone());
        }

        // get votes from other servers
        let mut votes = 0;
        for server in cluster.iter_mut() {
            if server.id == server.id {
                continue;
            }

            if server.voted_for == Some(server.id) {
                votes += 1;
            }
        }

        // if votes > n/2, change state to leader
        if votes > cluster.len() / 2 {
            self.state = ServerState::Leader;
            // reset election timeout
            self.election_timeout = rand::thread_rng().gen_range(150..300);
        } else {
            // failed election
            self.state = ServerState::Follower;
        }

        Self {
            id: self.id,
            current_term: self.current_term,
            voted_for: self.voted_for,
            log: self.log.clone(),
            commit_index: self.commit_index,
            state: self.state.clone(),
            election_timeout: self.election_timeout,
            hostname: self.hostname.clone(),
            port: self.port,
            listener: self.listener.try_clone().unwrap(),
        }
    }

    fn send_append_entries<T>(&mut self, x: Box<T>) -> Self {
        todo!()
    }

    fn send_request_vote(&mut self, request_vote_args: RequestVoteArgs) -> Self {
        todo!()
    }

    fn handle_append_entries(&mut self, mut cluster: Vec<ServerMetaData>) {
        // listen to server.port, tcp for append entries, don't crash if no messages, wait for election timeout
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            if stream.bytes().next().is_some() {
                // check if you are leader, if so then append to your log and send append log message to followers
                if self.state == ServerState::Leader {
                    // append to log
                    let mut reader = BufReader::new(stream);

                    // only add u64 to log
                    let mut buffer = [0; 8];
                    reader.read_exact(&mut buffer).unwrap();
                    let mut bytes = buffer.to_vec();
                    self.log.append(&mut bytes);

                    // send append entries to followers
                    let append_entries_args = AppendEntriesArgs {
                        term: self.current_term.clone(),
                        leader_id: self.id,
                        prev_log_index: self.log.len() as u64,
                        entries: self.log.clone(),
                        leader_commit: self.commit_index,
                    };

                    // send append entries to all other servers in cluster
                    for server in cluster.iter_mut() {
                        if server.id == server.id {
                            continue;
                        }

                        server.send_append_entries(Box::new(append_entries_args.clone()));
                    }
                }
            }
        }
    }

    fn handle_request_vote(&mut self) -> Self {
        todo!()
    }
}