use actix_web::{get, App, HttpServer, Responder};
use listenfd::ListenFd;

#[get("/")]
async fn greet() -> impl Responder {
    "hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| App::new().service(greet));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind("127.0.0.1:3000")?,
    };

    server.run().await
}
