use crate::model::post::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::PgConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

pub async fn add_post(json: web::Json<NewPost>, dbconn: web::Data<PgConnection>) -> impl Responder {
    let post = Post::create(&json.title, &json.body, &dbconn);
    HttpResponse::Ok().json(post)
}

pub async fn get_posts(dbconn: web::Data<PgConnection>) -> impl Responder {
    let posts = Post::find(&dbconn);
    HttpResponse::Ok().json(posts)
}

pub async fn get_post(
    web::Path(slug): web::Path<String>,
    dbconn: web::Data<PgConnection>,
) -> impl Responder {
    let post = Post::get(&slug, &dbconn);
    HttpResponse::Ok().json(post)
}

#[derive(Deserialize)]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub async fn update_post(
    web::Path(slug): web::Path<String>,
    json: web::Json<UpdatePost>,
    dbconn: web::Data<PgConnection>,
) -> impl Responder {
    let post = Post::update(&slug, &json.title, &json.body, json.published, &dbconn);
    HttpResponse::Ok().json(post)
}

pub async fn delete_post(
    web::Path(slug): web::Path<String>,
    dbconn: web::Data<PgConnection>,
) -> impl Responder {
    let deleted = Post::delete(&slug, &dbconn);
    HttpResponse::Ok().json(deleted)
}
