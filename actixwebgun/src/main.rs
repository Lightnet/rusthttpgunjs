//! Simple echo websocket server.
//! Open `http://localhost:8080/ws/index.html` in browser
//! or [python console client](https://github.com/actix/examples/blob/master/websocket/websocket-client.py)
//! could be used for testing.
#![warn(unused_imports)]
#![warn(dead_code)]
extern crate actix;
extern crate actix_web;
extern crate actix_web_actors;
extern crate actix_files;
extern crate gunrs;
//https://docs.rs/serde_json/0.9.0-rc2/serde_json/
//https://tutorialedge.net/rust/rust-working-with-json-tutorial/
//https://docs.serde.rs/serde_json/macro.json.html

extern crate serde_json;
//use std::ptr::null;

use serde_json::{Value};
//use serde_json::json;

use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

//use gunrs;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

//===============================================
// WEB SOCKET STRUCT / FUNCTIONS

/// do websocket handshake and start `MyWebSocket` actor
fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(), &r, stream);
    println!("{:?}", res.as_ref().unwrap());
    res
}
/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}
impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}
/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping("");
        });
    }
}
//===============================================
// GUN  STRUCT / FUNCTIONS

struct Key {

}

/// do websocket handshake and start `GunSocket` actor
fn gun_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(GunWebSocket::new(), &r, stream);
    println!("{:?}", res.as_ref().unwrap());
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct GunWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    #[allow(dead_code)]
    key: Key,//store keys ???
}
impl Actor for GunWebSocket {
    type Context = ws::WebsocketContext<Self>;
    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}
/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for GunWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                println!("text: {:?}", text);
                //https://docs.serde.rs/serde_json/value/enum.Value.html#method.as_str
                //let data: Value = serde_json::from_str("{\"foo\": 13, \"bar\": \"baz\"}").unwrap();
                //https://github.com/serde-rs/json
                let data: Value = serde_json::from_str(&text).unwrap();
                
                //println!("data: {:?}", data);
                //println!("data: {}", data["#"]);
                // data: {"bar":"baz","foo":13}
                //println!("object? {}", data.is_object());
                println!("data: {}", data["#"]);
                if data["#"] != Value::Null {
                    println!("Found! #");
                }else{
                    println!("Null #");
                }

                // object? true
                let obj = data.as_object().unwrap();

                //let foo = obj.get("#").unwrap();
                //println!("string? {:?}", foo.as_str());
                //println!("string? {:?}", foo);

                if obj.get("get") != None{
                    println!("get? Found!");
                    //self.dup(String::from("test call"));
                    //println!("{}", Value::String(data["#"].to_string()) );
                    println!("{}", data["#"].to_string() );
                    //self.dup(obj.get("#"));

                    self.dup(data["#"].to_string());
                }

                if obj.get("put") != None{
                    println!("put? Found!");
                }

                //let get = obj.get("get").unwrap();
                //println!("array? {:?}", foo.as_array());
                // array? None
                //println!("u64? {:?}", foo.as_u64());
                // u64? Some(13u64)
                //for (key, value) in obj.iter() {
                    //println!("{}: {}", key, match *value {
                        //Value::Number(ref v) => format!("{} (number)", v),
                        //Value::String(ref v) => format!("{} (string)", v),
                        //_ => format!("other")
                    //});
                //}
                //send to client?
                ctx.text(text);
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
    
}

impl GunWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now(), key: Key{} }//websocket, key (store list / fun keys)
    }
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping("");
        });
    }
    //duplication check for keys
    #[allow(dead_code)]
    fn dup(&mut self,data: String){
        println!("data hash #: {}", data);

        //println!("key {:#?}", self.key);
    }
}

fn main() -> std::io::Result<()> {
    //server code below
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(web::resource("/gun/").route(web::get().to(gun_index)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
}
