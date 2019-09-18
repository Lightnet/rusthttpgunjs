//https://doc.rust-lang.org/book/ch01-02-hello-world.html


// Note: `hyper::upgrade` docs link to this upgrade.
 #![warn(unused_variables)]
 #![warn(unused_imports)]
extern crate futures;
extern crate hyper;
extern crate tokio;

use std::str;
use std::io;

use futures::sync::oneshot;

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::header::{UPGRADE, HeaderValue};
use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;

static NOTFOUND: &[u8] = b"Not Found";
//static INDEX: &str = "index.html";
const HTML: &'static str = include_str!("clientwebsockets.html");
//type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=io::Error> + Send>;

/// Our server HTTP handler to initiate HTTP upgrades.
fn server_upgrade(req: Request<Body>) -> Response<Body> {
    println!("Query:>");
    println!("{}",req.uri());

    
    let mut res = Response::new(Body::empty());

    if !req.headers().contains_key("/") {
        //*res.status_mut() = INDEX;
        //*res.body_mut() = Body::from("Try POSTing data to /echo");
        *res.body_mut() = Body::from(HTML);
        return res;
    }

    // Send a 400 to any request that doesn't have
    // an `Upgrade` header.
    if !req.headers().contains_key(UPGRADE) {
        *res.status_mut() = StatusCode::BAD_REQUEST;
        return res;
    }

    let on_upgrade = req
        .into_body()
        .on_upgrade()
        .map_err(|err| eprintln!("upgrade error: {}", err))
        .and_then(|upgraded| {
            tokio::io::read_exact(upgraded, vec![0; 7])
                .and_then(|(upgraded, vec)| {
                    println!("server[foobar] recv: {:?}", str::from_utf8(&vec));
                    // And now write back the server 'foobar' protocol's
                    // response...
                    tokio::io::write_all(upgraded, b"bar=foo")
                })
                .map(|_| println!("server[foobar] sent"))
                .map_err(|e| eprintln!("server foobar io error: {}", e))
        });

    rt::spawn(on_upgrade);

    *res.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
    res.headers_mut().insert(UPGRADE, HeaderValue::from_static("foobar"));
    res
}

fn main() {
    // For this example, we just make a server and our own client to talk to
    // it, so the exact port isn't important. Instead, let the OS give us an
    // unused port.
    let addr = ([127, 0, 0, 1], 8080).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(server_upgrade))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);

    /*
    // We need the assigned address for the client to send it messages.
    let addr = server.local_addr();


    // For this example, a oneshot is used to signal that after 1 request,
    // the server should be shutdown.
    let (tx, rx) = oneshot::channel();

    let server = server
        .map_err(|e| eprintln!("server error: {}", e))
        .select2(rx)
        .then(|_| Ok(()));

    rt::run(rt::lazy(move || {
        rt::spawn(server);

        let req = Request::builder()
            .uri(format!("http://{}/", addr))
            .header(UPGRADE, "foobar")
            .body(Body::empty())
            .unwrap();

        Client::new()
            .request(req)
            .and_then(|res| {
                if res.status() != StatusCode::SWITCHING_PROTOCOLS {
                    panic!("Our server didn't upgrade: {}", res.status());
                }

                res
                    .into_body()
                    .on_upgrade()
            })
            .map_err(|e| eprintln!("client error: {}", e))
            .and_then(|upgraded| {
                // We've gotten an upgraded connection that we can read
                // and write directly on. Let's start out 'foobar' protocol.
                tokio::io::write_all(upgraded, b"foo=bar")
                    .and_then(|(upgraded, _)| {
                        println!("client[foobar] sent");
                        tokio::io::read_to_end(upgraded, Vec::new())
                    })
                    .map(|(_upgraded, vec)| {
                        println!("client[foobar] recv: {:?}", str::from_utf8(&vec));


                        // Complete the oneshot so that the server stops
                        // listening and the process can close down.
                        let _ = tx.send(());
                    })
                    .map_err(|e| eprintln!("client foobar io error: {}", e))
            })
    }));
    */
}