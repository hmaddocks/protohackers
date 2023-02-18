use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    println!("New client connected: {}", stream.peer_addr()?);

    let mut buffer = vec![0u8; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Client disconnected: {}", stream.peer_addr()?);
            return Ok(());
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received message: {}", message);

        stream.write_all(&buffer[..bytes_read])?;
    }
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        std::thread::spawn(|| handle_client(stream?));
    }
    Ok(())
}
