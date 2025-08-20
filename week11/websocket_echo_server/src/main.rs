use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    println!("🌐 WebSocket Echo Server running on ws://127.0.0.1:9001");
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    while let Ok((stream, addr)) = listener.accept().await {
        println!("🔌 New connection from: {}", addr);
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await;
            match ws_stream {
                Ok(mut websocket) => {
                    while let Some(msg) = websocket.next().await {
                        match msg {
                            Ok(msg) if msg.is_text() || msg.is_binary() => { 
                                println!("📨 Echoing message: {:?}", msg);
                                if let Err(e) = websocket.send(msg).await {
                                    eprintln!("❌ Error sending message: {}", e);
                                    break;
                                }
                            }
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("❌ Error receiving message: {}", e);
                                break;
                            }
                        }
                    }
                },
                Err(e) => eprintln!("❌ Handshake error: {}", e),
            }
        });
    }
}
