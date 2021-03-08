use actix_web::{post, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::postgres::PgPool;
use tracing::{event, instrument, Level};
use uuid::Uuid;

#[derive(Serialize)]
struct Urlet {
    id: Uuid,
    url: String,
}

#[instrument(skip(pool))]
#[post("/")]
pub async fn generate_urlet(url: String, pool: web::Data<PgPool>) -> impl Responder {
    let id = Uuid::new_v4();
    event!(Level::INFO, %id, %url, "inserting a new urlet into the database");
    let res = sqlx::query_as!(
        Urlet,
        r#"INSERT INTO urlet(id, url) VALUES ( $1, $2 ) RETURNING *"#,
        id,
        url
    )
    .fetch_one(pool.get_ref())
    .await;

    match res {
        Ok(urlet) => HttpResponse::Ok().json(urlet).await,
        _ => HttpResponse::InternalServerError().finish().await,
    }
}
