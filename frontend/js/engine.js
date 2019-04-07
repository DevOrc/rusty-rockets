var socket;
var msgs = []
var running = true;

export function initEngine() {
    var hostName = window.location.hostname;
    var port = window.location.port;

    var socketAddress = "ws:" + hostName + ":" + port + "/ws";

    console.log("Connecting web socket to " + socketAddress)
    socket = new WebSocket(socketAddress);

    socket.onopen = function (){
        console.log("Socket opened! Sending cached messages!");

        msgs.map(function(msg){
            send_message(msg)
        })
    }

    socket.onmessage = function (event) {
        console.log("Recieved message from server: " + event.data);
    };

    socket.onerror = onSocketError;
    socket.onclose = onSocketClose;
}

function onSocketClose(){
    console.log("Closing socket!")
    running = false;
}

function onSocketError(){
    console.log("Socket had an error!");
    onSocketClose();
}

export function send_message(text) {
    if (socket.readyState == 1) {
        console.log("Sending message over web socket: " + text);

        socket.send(text);
    } else if (socket.readyState == 0) {
        console.log("wating for socket to open before sending message: " + text)
       msgs.push(new String(text));
    } else {
        console.log("WARNING! WEB SOCKET NOT ABLE TO SEND MESSAGE!! State: " + socket.readyState);
    }
}

export function isRunning(){
    return running
}