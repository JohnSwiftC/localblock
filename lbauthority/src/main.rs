use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:2727").await.unwrap();
    
    loop {
        let socket = match listener.accept().await {
            Ok((s, _)) => s,
            Err(e) => {
                eprintln!("Failure on a client: {}", e); 
                continue;    
            }
        };

        tokio::spawn(async move {
            handle_stream(socket).await
        });
    }
}

async fn handle_stream(socket: TcpStream) {
    
}
