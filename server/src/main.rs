use warp::*;
use warp::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};

use futures::sync::mpsc;

fn main() {
    pretty_env_logger::init();

    let ws_route = warp::path("ws")
        .and(warp::ws2())
        .map(|ws: warp::ws::Ws2| {
            ws.on_upgrade(move |socket| client_connected(socket))
        });
        
    let file_route = warp::fs::dir("assets");

    let routes = ws_route.or(file_route);
        
    warp::serve(routes).run(([0, 0, 0, 0], 8080));
}

fn client_connected(ws: WebSocket) -> impl Future<Item = (), Error = ()> {
    println!("Client connected!");

    let (ws_tx, ws_rx) = ws.split();

    //Create channel to use for web socket tx
    let (tx, rx) = mpsc::unbounded();
    warp::spawn(
        rx.map_err(|()| -> warp::Error { unreachable!("unbounded rx never errors") })
            .forward(ws_tx)
            .map(|_| ())
            .map_err(|e| eprintln!("[Websocket Send Edrror]: {}", e)),
    );

    let tx = Arc::new(Mutex::new(tx));

    ws_rx.for_each(move |msg| { // Handle Message
        println!("Message Recieved: {:?}", msg);

        tx.lock().unwrap().unbounded_send(msg).unwrap();
        Ok(())
    })
    .then(move |result| { // On Disconnect
        println!("Client disconnected!");
        result
    })
    .map_err(move |e| {
        eprintln!("[Web Socket Error]: {}", e);
    })
}