#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod api;
pub mod db;
pub mod model;
use crate::api::post::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    HttpServer::new(move || {
        App::new()
            .data(
                PgConnection::establish(&database_url)
                    .expect(&format!("Error connecting to {}", &database_url)),
            )
            .service(
                web::scope("/post")
                    .service(
                        web::resource("")
                            .route(web::post().to(add_post))
                            .route(web::get().to(get_posts)),
                    )
                    .service(
                        web::resource("{slug}")
                            .route(web::get().to(get_post))
                            .route(web::put().to(update_post))
                            .route(web::delete().to(delete_post)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
