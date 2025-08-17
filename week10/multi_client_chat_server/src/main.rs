use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

fn main() -> std::io::Result<()> {
    println!("💬 Multi-Client Chat Server listening on 127.0.0.1:7878");
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let addr = stream.peer_addr()?.to_string();
        println!("🔗 New connection: {}", addr);
        let clients = Arc::clone(&clients);
        clients
            .lock()
            .unwrap()
            .insert(addr.clone(), stream.try_clone()?);
        println!("add connected {}.", &addr);
        thread::spawn(move || handle_client(stream, addr, clients));
    }
    Ok(())
}

fn handle_client(stream: TcpStream, addr: String, clients: Clients) {
    let reader = BufReader::new(stream.try_clone().unwrap());
    for line in reader.lines() {
        println!("start to read from {}.", &addr);
        let msg = match line {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("❌ Error reading from {}: {}", addr, e);
                break;
            }
        };
        let full_msg = format!("[{}]: {}", addr, msg);
        println!("{}", full_msg);
        let mut clients_lock = clients.lock().unwrap();
        let mut disconnected = vec![];
        for (peer, mut clinet_stream) in clients_lock.iter_mut() {
            println!("disconnected peer: {}", &peer);
            println!("disconnected addr: {}", &addr);
            if peer != &addr {
                if let Err(_) = writeln!(clinet_stream, "{}", full_msg) {
                    println!("disconnected remove: {}.", &peer);
                    disconnected.push(peer.clone());
                }
            }
        }
        for peer in disconnected {
            clients_lock.remove(&peer);
        }
        println!("one iteration finished to read from {}.", &addr);
    }
    println!("❌ {} disconnected.", addr);
    clients.lock().unwrap().remove(&addr);
}
