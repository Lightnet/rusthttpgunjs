#[macro_use] 
extern crate log;
extern crate env_logger;
extern crate tungstenite;

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::spawn;

use tungstenite::{accept, HandshakeError, Error, Result, Message};
use tungstenite::handshake::HandshakeRole;

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(stream: TcpStream) -> Result<()> {
    println!("incoming...");
    let mut socket = accept(stream).map_err(must_not_block)?;
    loop {
        match socket.read_message()? {
            //msg @ Message::Text(_) |
            msg @ Message::Text(_) =>{
                println!("msg: {}", msg);
                socket.write_message(msg)?;
            }
            msg @ Message::Binary(_) => {
                socket.write_message(msg)?;
            }
            Message::Ping(_) |
            Message::Pong(_) |
            Message::Close(_) => {}
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for http://localhost:{}", 8080);

    //thread::spawn(move || {
        //for stream in server.incoming() {
            //let hstream = stream.unwrap();
		    //handle_connection(hstream);
        //}
    //});

    for stream in server.incoming() {
        spawn(move || {
            match stream {
                Ok(stream) => match handle_client(stream) {
                    Ok(_) => (),
                    Err(e) => warn!("Error in client: {}", e),
                },
                Err(e) => warn!("Error accepting stream: {}", e),
            }
        });
        
        
    }
}


