// main.rs

use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter};
use std::net::SocketAddr;
use std::sync::Arc;
use std::net::TcpStream;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex as TokioMutex};
use tokio::time::{self, Duration};
use std::io::Write;

#[derive(Debug, Clone)]
enum Command {
    Put(String, String),
    Get(String),
    Del(String),
}

#[derive(Debug, Clone)]
enum Message {
    Command(Command),
    NewNode(SocketAddr),
    Election(SocketAddr),
    Leader(SocketAddr),
    Heartbeat(SocketAddr),
}

const HEARTBEAT_INTERVAL_SECONDS: u64 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    let (tx, _) = broadcast::channel::<Message>(1024);

    let store = Arc::new(TokioMutex::new(HashMap::new()));
    let leader = Arc::new(TokioMutex::new(None));
    let nodes = Arc::new(TokioMutex::new(Vec::new()));

    let store_clone = Arc::clone(&store);
    let tx_clone = tx.clone();
    let leader_clone = Arc::clone(&leader);
    let nodes_clone = Arc::clone(&nodes);

    // Listen for new nodes joining
    tokio::spawn(async move {
        loop {
            if let Ok((socket, _)) = listener.accept().await {
                let addr = socket.peer_addr().unwrap();
                tx.send(Message::NewNode(addr)).unwrap();

                let store_clone = Arc::clone(&store);
                let tx_clone = tx.clone();
                let leader_clone = Arc::clone(&leader);
                let nodes_clone = Arc::clone(&nodes);
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(socket_clone, store_clone, tx_clone, leader_clone).await {
                        eprintln!("Error handling connection: {}", err);
                        let mut nodes = nodes_clone.lock().await;
                        nodes.retain(|&x| x != addr); // Remove failed node from the nodes list
                    }
                });
            }
        }
    });

    // Process incoming messages
    let nodes_clone_heartbeat = Arc::clone(&nodes);
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECONDS));
        loop {
            interval.tick().await;
            let nodes = nodes_clone_heartbeat.lock().await;
            for &node in &*nodes {
                tx.send(Message::Heartbeat(node)).unwrap();
            }
        }
    });

    loop {
        let message = tx.subscribe().recv().await.unwrap();
        match message {
            Message::Command(command) => {
                handle_command(command, Arc::clone(&store)).await?;
            }
            Message::NewNode(addr) => {
                println!("New node joined: {}", addr);
                let mut nodes = nodes.lock().await;
                nodes.push(addr); // Add new node to the nodes list
                if leader.lock().await.is_none() {
                    tx.send(Message::Election(addr)).unwrap();
                }
            }
            Message::Election(addr) => {
                let current_leader = leader.lock().await.clone();
                if current_leader.is_none() || addr.to_string() > current_leader.unwrap().to_string() {
                    leader.lock().await.replace(addr);
                    tx.send(Message::Leader(addr)).unwrap();
                }
            }
            Message::Leader(addr) => {
                println!("New leader elected: {}", addr);
                leader.lock().await.replace(addr);
            }
            Message::Heartbeat(addr) => {
                println!("Heartbeat received from node: {}", addr);
            }
        }
    }
}


async fn handle_connection(
    socket: TcpStream,
    store: Arc<TokioMutex<HashMap<String, String>>>,
    tx: broadcast::Sender<Message>,
    _leader: Arc<TokioMutex<Option<SocketAddr>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket_clone = socket.try_clone().expect("Clone failed ...");
    let (reader, writer) = (socket_clone, socket);

    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    let mut buffer = String::new();

    // Read the request from the client
    reader.read_line(&mut buffer)?;

    // Parse the request
    let parts: Vec<&str> = buffer.trim().split(' ').collect();
    let response = match parts.len() {
        2 if parts[0] == "GET" => {
            let key = parts[1];
            let store = store.lock().await;
            match store.get(key) {
                Some(value) => format!("OK {}\n", value),
                None => "NOT FOUND\n".to_string(),
            }
        }
        3 if parts[0] == "PUT" => {
            let key = parts[1];
            let value = parts[2];
            store.lock().await.insert(key.to_string(), value.to_string());
            tx.send(Message::Command(Command::Put(key.to_string(), value.to_string())))
                .unwrap();
            "OK\n".to_string()
        }
        2 if parts[0] == "DEL" => {
            let key = parts[1];
            store.lock().await.remove(key);
            tx.send(Message::Command(Command::Del(key.to_string()))).unwrap();
            "OK\n".to_string()
        }
        _ => "ERROR\n".to_string(),
    };

    // Write the response to the client
    writer.write_all(response.as_bytes())?;
    writer.flush()?;

    Ok(())
}

async fn handle_command(command: Command, store: Arc<TokioMutex<HashMap<String, String>>>) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Put(key, value) => {
            store.lock().await.insert(key, value);
        }
        Command::Get(_) => {
            // GET command is handled within handle_connection
        }
        Command::Del(key) => {
            store.lock().await.remove(&key);
        }
    }
    Ok(())
}
