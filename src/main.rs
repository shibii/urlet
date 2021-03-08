use actix_web::{get, App, HttpServer, Responder};
use listenfd::ListenFd;
use tracing::{event, instrument, Level};

mod dotenv;

#[instrument(name = "get /")]
#[get("/")]
async fn greet(req_body: String) -> impl Responder {
    event!(Level::INFO, "get /");
    format!("hello {}", req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::parse();
    tracing_subscriber::fmt::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(greet));

    let bind_addr = std::env::var("SERVER_BIND_ADDRESS")
        .expect("missing environment variable: SERVER_BIND_ADDRESS");

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(bind_addr)?,
    };

    server.run().await
}
