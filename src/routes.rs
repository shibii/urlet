use actix_web::{get, Responder};
use tracing::{event, instrument, Level};

#[instrument(name = "get /")]
#[get("/")]
pub async fn greet(req_body: String) -> impl Responder {
    event!(Level::INFO, "get /");
    format!("hello {}", req_body)
}
