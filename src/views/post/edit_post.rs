use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use diesel::RunQueryDsl;

use crate::json_serialization::post::Post as Json_Response;
use crate::jwt::JwtToken;
use crate::database::DB;
use crate::schema::post;
use crate::models::post::post::Post;

pub async fn edit_post(post_request: web::Json<Json_Response>, req: HttpRequest, token: JwtToken, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();

    let update_result = diesel::update(
        post::table.filter(post::columns::user_id.eq(token.user_id.clone()))
        .filter(post::columns::id.eq(id.parse::<i32>().unwrap()))
    )
        .set((post::columns::title.eq(&post_request.title), (post::columns::content.eq(&post_request.content))))
        .get_result::<Post>(&db.connection);

    match update_result {
        Ok(updated_post) => {
            let json = Json_Response {
                id: Some(updated_post.id),
                title: updated_post.title,
                content: updated_post.content,
                date: Some(updated_post.date),
            };
            return HttpResponse::Ok().json(json);
        },
        Err(_) => return HttpResponse::NotFound().into()
    }
}