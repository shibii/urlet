use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use tracing::{event, Level};

mod db;
mod dotenv;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::parse();
    tracing_subscriber::fmt::init();
    let mut listenfd = ListenFd::from_env();

    let db_addr =
        std::env::var("DATABASE_URL").expect("missing environment variable: DATABASE_URL");

    event!(Level::INFO, "connecting to database");
    let pool = db::connect(&db_addr)
        .await
        .expect("failed to connect to the database");

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(routes::generate_urlet)
            .service(routes::redirect)
    });

    let bind_addr = std::env::var("SERVER_BIND_ADDRESS")
        .expect("missing environment variable: SERVER_BIND_ADDRESS");

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(bind_addr)?,
    };

    server.run().await
}
