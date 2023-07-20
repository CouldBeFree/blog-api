use actix_web::{web, Responder, HttpResponse};
use diesel::RunQueryDsl;

use crate::json_serialization::post::Post;
use crate::jwt::JwtToken;
use crate::database::DB;
use crate::models::post::new_post::NewPost;
use crate::schema::post;

pub async fn create_post(post: web::Json<Post>, token: JwtToken, db: DB) -> impl Responder {
    let new_post = NewPost::new(
        post.title.clone(),
        post.content.clone(),
        token.user_id.clone()
    );

    println!("token, {}", &token.user_id);

    let _ = diesel::insert_into(post::table)
        .values(&new_post)
        .execute(&db.connection);

    return HttpResponse::Ok()
}