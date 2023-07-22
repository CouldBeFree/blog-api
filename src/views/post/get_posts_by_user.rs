use crate::diesel;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{Responder, HttpResponse};

use crate::jwt::JwtToken;
use crate::models::post::post::Post;
use crate::json_serialization::post::Post as Json_Response;
use crate::database::DB;
use crate::schema::post;

pub async fn get_posts_by_user(token: JwtToken, db: DB) -> impl Responder {
    let posts = post::table
        .filter(post::columns::user_id.eq(token.user_id.clone()))
        .load::<Post>(&db.connection);

    match posts {
        Ok(res) => {
            let json_responses: Vec<Json_Response> = res.into_iter()
                .map(|post| Json_Response {
                    id: Some(post.id),
                    title: post.title,
                    content: post.content,
                    date: Some(post.date),
                }).collect();
            return HttpResponse::Ok().json(json_responses)
        }
        Err(_) => return HttpResponse::NotFound().into()
    }
}