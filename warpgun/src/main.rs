#![deny(warnings)]
#![deny(unused_imports)]
extern crate futures;
extern crate pretty_env_logger;
extern crate warp;

use warp::{Filter, Future, Stream};

fn main() {
    pretty_env_logger::init();
    // Match any request and return hello world!
    //let routes = warp::any().map(|| "Hello, World!");

    let echo = warp::path("echo")
        // The `ws2()` filter will prepare the Websocket handshake.
        .and(warp::ws2())
        .map(|ws: warp::ws::Ws2| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|_| ()).map_err(|e| {
                    eprintln!("websocket error: {:?}", e);
                })
            })
        });

    // GET / -> index html
    //let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    //let routes = index.or(echo);

    warp::serve(echo).run(([127, 0, 0, 1], 8080));
}
/*
static INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        var uri = 'ws://' + location.host + '/chat';
        var ws = new WebSocket(uri);
        function message(data) {
            var line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }
        ws.onopen = function() {
            chat.innerHTML = "<p><em>Connected!</em></p>";
        }
        ws.onmessage = function(msg) {
            message(msg.data);
        };
        send.onclick = function() {
            var msg = text.value;
            ws.send(msg);
            text.value = '';
            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;
*/