use sqlite::Connection;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::{JoinHandle};
use std::sync::{Arc, Mutex};

mod database;

use lbcrypto;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:2727").await.unwrap();
    let connection = database::init_db_conn("auth.db").unwrap();

    loop {
        let socket = match listener.accept().await {
            Ok((s, _)) => s,
            Err(e) => {
                eprintln!("Failure on a client: {}", e);
                continue;
            }
        };

        let h = tokio::spawn(async move { handle_stream(socket).await });
    }
}

async fn handle_stream(socket: TcpStream) {}

/// Holds join hand
struct NodeContext {
    worker: Arc<Mutex<JoinHandle<()>>>,
}

async fn search_for_nonce() {}