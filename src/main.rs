use actix_web::{error, web, App, HttpResponse, HttpServer, Responder, get};
mod api;
// use api::auth::{login, register, forget_password, test};
mod schema;
mod model;
mod errors;
mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{pg::PgConnection, r2d2::{self, ConnectionManager}};
use dotenv::dotenv;
use std::env;

use crate::api::auth::register::register_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: model::db::Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .app_data(
            // Json extractor configuration for resources.
            web::JsonConfig::default().error_handler(|err, _req| {
                let e = format!("{:?}", err);
                error::InternalError::from_response(err, HttpResponse::BadRequest().body(e))
                    .into()
            }),
        )
        .service(
            web::resource("/register")
            .route(web::post().to(register_handler))
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
