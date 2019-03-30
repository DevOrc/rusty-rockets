fn main() {
    warp::serve(warp::fs::dir("assets")).run(([127, 0, 0, 1], 8080));
}
