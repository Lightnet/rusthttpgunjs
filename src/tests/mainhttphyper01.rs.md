

//https://rust-lang.github.io/async-book/01_getting_started/05_http_server_example.html
//https://rust-lang.github.io/async-book/02_execution/02_future.html
#![deny(warnings)]
#![allow(unused_variables)]
extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
//use hyper::service::service_fn_ok;

//const PHRASE: &str = "Hello, World! Rust lang!";

const BAR: &str = "Hello, World! Rust lang!";

static mut indexhtml: String = "".to_string();
static mut clientjs: String = "".to_string();

//fn hello_world(_req: Request<Body>) -> Response<Body> {
    //Response::new(Body::from(PHRASE))
//}

// Just a simple type alias
type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::GET, "/foo") => {
            //*response.body_mut() = Body::from("Try POSTing data to /echo");
            *response.body_mut() = Body::from(BAR);
        },
        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req.into_body().map(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });

            *response.body_mut() = Body::wrap_stream(mapping);
        },
        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive. So, this returns a different
        // future, waiting on concatenating the full body, so that
        // it can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let reversed = req.into_body().concat2().map(move |chunk| {
                let body = chunk.iter().rev().cloned().collect::<Vec<u8>>();
                *response.body_mut() = Body::from(body);
                response
            });

            return Box::new(reversed);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Box::new(future::ok(response))
}


fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    //let new_svc = || {
        //
        //service_fn_ok(hello_world)
        //service_fn_ok(|_req|{
            //Response::new(Body::from("Hello, World! Rust!"))
        //})
    //};

    let server = Server::bind(&addr)
        //.serve(new_svc)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}