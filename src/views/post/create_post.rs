use actix_web::{web, Responder, HttpResponse};
use diesel::RunQueryDsl;

use crate::json_serialization::post::Post;
use crate::jwt::JwtToken;
use crate::database::DB;
use crate::models::post::new_post::NewPost;
use crate::schema::post;
use crate::models::post::post::Post as Post_Model;

pub async fn create_post(post: web::Json<Post>, token: JwtToken, db: DB) -> impl Responder {
    let new_post = NewPost::new(
        post.title.clone(),
        post.content.clone(),
        token.user_id.clone()
    );

    let insert_result = diesel::insert_into(post::table)
        .values(&new_post)
        .get_result::<Post_Model>(&db.connection);

    match insert_result {
        Ok(post) => {
            let json = Post {
                id: Some(post.id),
                title: post.title,
                content: post.content,
                date: Some(post.date),
            };
            return HttpResponse::Ok().json(json)
        },
        Err(_) => return HttpResponse::InternalServerError().into()
    }
}