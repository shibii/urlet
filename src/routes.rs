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

#[allow(clippy::async_yields_async)]
#[instrument(skip(req, pool))]
#[get("/{id}")]
async fn redirect(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    let id = match req.match_info().get("id") {
        Some(id) => id,
        _ => return HttpResponse::BadRequest().finish(),
    };

    let uuid = match super::urlet::decode(id) {
        Ok(uuid) => uuid,
        _ => return HttpResponse::BadRequest().finish(),
    };

    let res = sqlx::query_as!(Urlet, r#"SELECT * FROM urlet WHERE id = $1"#, uuid)
        .fetch_one(pool.get_ref())
        .await;

    match res {
        Ok(urlet) => HttpResponse::PermanentRedirect()
            .header("Location", urlet.url)
            .finish(),
        _ => HttpResponse::BadRequest().finish(),
    }
}

#[allow(clippy::async_yields_async)]
#[instrument(skip(pool))]
#[post("/")]
async fn generate_urlet(url: String, pool: web::Data<PgPool>) -> impl Responder {
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
        Ok(_) => HttpResponse::Ok().json(urlet),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(redirect);
    cfg.service(generate_urlet);
}
