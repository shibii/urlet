use actix_web::{App, HttpServer};
use listenfd::ListenFd;

mod dotenv;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::parse();
    tracing_subscriber::fmt::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(routes::greet));

    let bind_addr = std::env::var("SERVER_BIND_ADDRESS")
        .expect("missing environment variable: SERVER_BIND_ADDRESS");

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(bind_addr)?,
    };

    server.run().await
}
