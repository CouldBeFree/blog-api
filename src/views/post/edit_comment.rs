use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use diesel::RunQueryDsl;

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::schema::comment;
use crate::models::comment::comment::Comment;
use crate::json_serialization::comment::Comment as Json_Comment;

pub async fn edit_comment(req: HttpRequest, comment: web::Json<Json_Comment>, token: JwtToken, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();
    let post_id = id.parse::<i32>().unwrap();

    let comment_id = req.match_info().get("comment_id").unwrap().to_string();
    let comment_id = comment_id.parse::<i32>().unwrap();

    let update_result = diesel::update(
        comment::table.filter(comment::columns::user_id.eq(token.user_id.clone()))
        .filter(comment::columns::post_id.eq(post_id))
        .filter(comment::columns::id.eq(comment_id))
    )
        .set(comment::columns::content.eq(&comment.content))
        .get_result::<Comment>(&db.connection);

    match update_result {
        Ok(res) => {
            let json = Json_Comment {
                id: Some(res.id),
                date: Some(res.date),
                content: res.content
            };
            return HttpResponse::Ok().json(json);
        },
        Err(_) => return HttpResponse::NotFound().into()
    }
}