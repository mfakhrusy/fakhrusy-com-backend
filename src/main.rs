use actix_web::{error, web, App, HttpResponse, HttpServer};
mod api;
mod constants;
mod extractor;
mod middleware;
mod model;
mod schema;
mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenv::dotenv;
use std::env;

use crate::api::auth::login::login_handler;
use crate::api::auth::register::register_handler;
use crate::api::profile::my_profile::my_profile_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: model::db::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(crate::middleware::auth::Authentication)
            .app_data(
                // Json extractor configuration for resources.
                web::JsonConfig::default().error_handler(|err, _req| {
                    let e = format!("{:?}", err);
                    error::InternalError::from_response(err, HttpResponse::BadRequest().body(e))
                        .into()
                }),
            )
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/auth")
                            .service(
                                web::resource("/register").route(web::post().to(register_handler)),
                            )
                            .service(web::resource("/login").route(web::post().to(login_handler))),
                    )
                    .service(web::resource("/profile").route(web::get().to(my_profile_handler))),
            )
    })
    .bind("127.0.0.1:8089")?
    .run()
    .await
}
