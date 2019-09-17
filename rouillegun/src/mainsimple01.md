use rouille::Request;
use rouille::Response;

rouille::start_server("0.0.0.0:80", move |request| {
    Response::text("hello world")
});