use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
// use std::thread;

const HOST: &str = "localhost:8080";
const FILE: &str = "public/index.html";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = TcpListener::bind(HOST)?;
    let file = Arc::new(fs::read_to_string(FILE)?);
    println!("Listening in http://{} serving {}", HOST, FILE);

    loop {
        let (stream, _) = socket.accept()?;
        handle_connection(stream, file.clone())?
    }
}

fn handle_connection(
    mut stream: TcpStream,
    content: Arc<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = BufReader::new(&stream);
    let request: Vec<String> = buffer
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if request[0] == "GET / HTTP/1.1" {
        stream.write_all(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            )
            .as_bytes(),
        )?;
    } else {
        stream.write_all(
            "HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request".as_bytes(),
        )?;
    }
    Ok(())
}
