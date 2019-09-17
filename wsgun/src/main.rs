extern crate ws;
//https://github.com/housleyjk/ws-rs/blob/stable/examples/server.rs
extern crate env_logger;

use ws::listen;

fn main () {

    // Setup logging
    env_logger::init();

    // Listen on an address and call the closure for each connection
    if let Err(error) = listen("127.0.0.1:8080", |out| {

        // The handler needs to take ownership of out, so we use move
        move |msg| {

            // Handle messages received on this connection
            println!("Server got message '{}'. ", msg);

            // Use the out channel to send messages back
            out.send(msg)
        }

    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }

}