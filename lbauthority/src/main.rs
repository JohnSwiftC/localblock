use tokio::net::{TcpListener, TcpStream};
use tokio::task::{JoinHandle};
use std::sync::{Arc, Mutex};

mod database;

use lbcrypto::{BlockHeader, Block, Transaction};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:2727").await.unwrap();
    let connection = database::init_db_conn("auth.db").unwrap();
    let search_task = tokio::task::spawn(async {

    });

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

async fn start_block_hashing(mut block: Block) -> JoinHandle<Block> {
    tokio::task::spawn(async move {
        block.search_for_nonce(27).await;
        // At this point we would broadcast this block
        block
    })
}

/// Holds join hand
struct NodeContext {
    worker: Arc<Mutex<JoinHandle<()>>>,
}


#[cfg(test)]
mod tests {
    use tokio::time::Instant;

    use super::*;

    #[tokio::test]
    async fn time_proof_of_work() {
        let mut block_header = BlockHeader::new(1, None);
        let txs = vec![Transaction::test_dummy(), Transaction::test_dummy(), Transaction::test_dummy(), Transaction::test_dummy(), Transaction::test_dummy(), Transaction::test_dummy()];
        block_header.compute_merkle_root(&txs);

        let mut block = Block {
            header: block_header,
            transactions: txs,
        };

        for i in 1..=40 {
            let start = Instant::now();
            block.search_for_nonce(i).await;
            block.header.set_nonce(0);
            let duration = start.elapsed();
            println!("Difficulty: {} : Time: {:?}", i, duration);
        }
    }
}