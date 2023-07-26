use crate::diesel;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use diesel::RunQueryDsl;

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::schema::comment;
use crate::models::comment::comment::Comment;
use crate::json_serialization::comment::Comment as Json_Comment;
use crate::models::comment::new_comment::NewComment;

pub async fn create_comment(req: HttpRequest, comment: web::Json<Json_Comment>, token: JwtToken, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();
    let parsed_id = id.parse::<i32>().unwrap();
    let new_comment = NewComment::new(
        comment.content.clone(),
        parsed_id.clone(),
        token.user_id.clone(),
    );

    let insert_result = diesel::insert_into(comment::table)
        .values(&new_comment)
        .get_result::<Comment>(&db.connection);

    match insert_result {
        Ok(_) => return HttpResponse::Ok(),
        Err(_) => return HttpResponse::NotFound()
    }
}