use actix_web::{get, App, HttpServer, Responder};
use dotenv_parser::parse_dotenv;
use listenfd::ListenFd;
use std::collections::BTreeMap;
use std::fs;

#[get("/")]
async fn greet() -> impl Responder {
    "hello"
}

fn get_env() -> BTreeMap<String, String> {
    let env_file = fs::read_to_string(".env").expect(".env file not found");
    parse_dotenv(&env_file).expect("failed to parse .env file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let env = get_env();
    let mut server = HttpServer::new(|| App::new().service(greet));

    let bind_addr = env
        .get("SERVER_BIND_ADDRESS")
        .expect("missing SERVER_BIND_ADDRESS env var");

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(bind_addr)?,
    };

    server.run().await
}
