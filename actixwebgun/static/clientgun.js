$(function() {

    function log(msg) {
        console.log(msg);
    }

    log("init gunjs websocket 0.02");

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
        log("test");
        var msg = {
            "#":"ASDF",
            message:"test"
        };
        //conn.send(msg);
        conn.send(JSON.stringify(msg));
    });

    $('#btnput').click(function() {
        log("put");
        var msg = {
            "#":"ASDF",
            put:{name:"foo"}
        };
        conn.send(JSON.stringify(msg));
    });

    $('#btnget').click(function() {
        console.log("get");
        log("get");
        var msg = {
            "#":"ASDF",
            get:{name:"foo"}
        };
        conn.send(JSON.stringify(msg));
    });

    $('#btngnull').click(function() {
        console.log("null");
        log("get");
        //var msg = {
            //"#":"ASDF",
            //get:{name:"foo"}
        //};
        //conn.send(JSON.stringify(msg));
    });
});