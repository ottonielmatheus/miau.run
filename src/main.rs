mod request;
mod response;
mod router;

use std::net::TcpListener;

use request::Meowquest;
use response::{*, StatusCode};
use router::Router;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("\n\n🦀 Listening to http://127.0.0.1:7878");

    let mut router = Router::new();

    router.get("/", &index);
    router.post("/", &api_index);

    router.get("*", &not_found);

    for stream in listener.incoming() {
        let req = Meowquest::new(stream.unwrap());
        println!("📨 received: {:?}", req);
        router.run(req);
    }
}

fn index(res: &mut Meowsponse) {
    res.status(StatusCode::Ok).html("index.html");
}

fn api_index(res: &mut Meowsponse) {
    res.status(StatusCode::Accepted).text("Meow!".into());
}

fn not_found(res: &mut Meowsponse) {
    res.status(StatusCode::NotFound).html("not-found.html");
}