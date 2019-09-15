$(function() {
    console.log("init gunjs websocket");
    function log(msg) {
        console.log(msg);
    }
    var conn = null;
    var wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/gun/';
    console.log(wsUri);
    conn = new WebSocket(wsUri);
    log('Connecting...');
    conn.onopen = function() {
        log('Connected.');
        //update_ui();
    };
    conn.onmessage = function(e) {
        log('Received: ' + e.data);
    };
    conn.onclose = function() {
        log('Disconnected.');
        conn = null;
        //update_ui();
    };

    $('#btntest').click(function() {
        var msg = {
            "#":"AWSDF",
            message:"test"
        };
        //conn.send(msg);
        conn.send(JSON.stringify(msg));

    });

    $('#btnput').click(function() {

    });

    $('#btnpget').click(function() {

    });
});