use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use sqlx::postgres::PgPool;
use tracing::{event, instrument, Level};
use uuid::Uuid;

#[derive(Serialize)]
struct Urlet {
    id: Uuid,
    url: String,
}

#[instrument(skip(req, pool))]
#[get("/{id}")]
pub async fn redirect(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    let id: &str = req.match_info().get("id").unwrap();
    let uuid = super::urlet::decode(id).unwrap();
    event!(Level::INFO, %id, "querying urlet from the database");
    let res = sqlx::query_as!(Urlet, r#"SELECT * FROM urlet WHERE id = $1"#, uuid)
        .fetch_one(pool.get_ref())
        .await;

    match res {
        Ok(urlet) => {
            HttpResponse::PermanentRedirect()
                .header("Location", urlet.url)
                .finish()
                .await
        }
        _ => HttpResponse::NotFound().finish().await,
    }
}

#[instrument(skip(pool))]
#[post("/")]
pub async fn generate_urlet(url: String, pool: web::Data<PgPool>) -> impl Responder {
    let uuid = Uuid::new_v4();
    event!(Level::INFO, %uuid, %url, "inserting a new urlet into the database");
    let res = sqlx::query_as!(
        Urlet,
        r#"INSERT INTO urlet(id, url) VALUES ( $1, $2 ) RETURNING *"#,
        uuid,
        url
    )
    .fetch_one(pool.get_ref())
    .await;

    let urlet = super::urlet::encode(uuid);

    match res {
        Ok(_) => HttpResponse::Ok().json(urlet).await,
        _ => HttpResponse::InternalServerError().finish().await,
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(redirect);
    cfg.service(generate_urlet);
}
