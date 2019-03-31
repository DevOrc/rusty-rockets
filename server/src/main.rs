use warp::*;

fn main() {
    pretty_env_logger::init();

    let ws_route = warp::path("ws")
        .and(warp::ws2())
        .map(|ws: warp::ws::Ws2| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|_| ()).map_err(|e| {
                    println!("websocket error: {:?}", e);
                })
            })
        });    
        
    let file_route = warp::fs::dir("assets");

    let routes = ws_route.or(file_route);
        
    warp::serve(routes).run(([0, 0, 0, 0], 8080));
}
