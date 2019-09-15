console.log("Hello World!")
var input = document.getElementById("input");
var output = document.getElementById("output");
//var socket = new WebSocket("ws://localhost:8080/echo");
var socket = new WebSocket("ws://localhost:8080/gun");

socket.onopen = function () {
    output.innerHTML += "Status: Connected\n";
};

socket.onmessage = function (e) {
    //output.innerHTML += "Server: " + e.data + "\n";
    var msg = JSON.parse(e.data);
    console.log(msg)
};

function send() {
    var msg = {
        '#': "1"
    }
    //socket.send(input.value);
    socket.send(JSON.stringify(msg));
    input.value = "";
}

function getsend() {
    var msg = {
        '#': "1",
        get:"test"
    }
    //socket.send(input.value);
    socket.send(JSON.stringify(msg));
    input.value = "";
}

function putsend() {
    var msg = {
        '#': "1",
        put:"test"
    }
    //socket.send(input.value);
    socket.send(JSON.stringify(msg));
    input.value = "";
}