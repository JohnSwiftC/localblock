use sqlite::Connection;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::{JoinHandle};
use std::sync::{Arc, Mutex};

mod database;

use lbcrypto::{BlockHeader, Block, Transaction};

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

/// Given a block head, will search for a nonce that hashes with the required 0 bits.
async fn search_for_nonce(mut block_header: BlockHeader, zero_bits: u8) -> BlockHeader {
    let mut attempt = 0;
    loop {
        let hash = block_header.hash();
        if hash.has_zero_bits(zero_bits) {
            return block_header;
        }

        attempt += 1;
        block_header.increment_nonce();
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::Instant;

    use super::*;

    #[tokio::test]
    async fn time_proof_of_work() {
        let mut block_header = BlockHeader::new(1, None);
        let txs = vec![Transaction::test_dummy(), Transaction::test_dummy(), Transaction::test_dummy()];
        block_header.compute_merkle_root(&txs);

        for i in 1..=30 {
            let start = Instant::now();
            block_header = search_for_nonce(block_header, i).await;
            block_header.set_nonce(0);
            let duration = start.elapsed();
            println!("Difficulty: {} : Time: {:?}", i, duration);
        }
    }
}