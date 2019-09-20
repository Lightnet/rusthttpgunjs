#![feature(proc_macro_hygiene, decl_macro)]
#![warn(unused_imports)]
#[macro_use] extern crate rocket;
extern crate ws;
//extern crate env_logger;

use std::thread;
use ws::listen;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    // Setup logging
    env_logger::init();
    //init web socket
    thread::spawn( move || {
        println!("init websocket");    
        listen("127.0.0.1:8080", |out| {
            move |msg| {
                out.send(msg)
            }
        })
    });

    println!("init web server");
    //init web server
    rocket::ignite().mount("/", routes![index]).launch();
}