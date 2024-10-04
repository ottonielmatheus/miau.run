mod request;
mod response;
mod router;

use std::net::TcpListener;

use request::Meowquest;
use response::{*, StatusCode};
use router::Router;

fn main() {
    handle_http();
}

fn handle_http() {
    let listener =
    TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("\n\n🦀 Listening to 127.0.0.1:7878...");

    let mut router = Router::new();

    router.get("/", &|res| index(res));
    router.get("*", &|res| not_found(res));

    for stream in listener.incoming() {
        router.run(Meowquest::new(stream.unwrap()));
    }
}

fn index (res: &mut Meowsponse) {
    res.status(StatusCode::Ok).html("index.html");
}

fn not_found (res: &mut Meowsponse) {
    res.status(StatusCode::NotFound).html("not-found.html");
}