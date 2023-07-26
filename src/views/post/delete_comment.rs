use crate::diesel;
use diesel::prelude::*;
use actix_web::{Responder, HttpResponse, HttpRequest};
use diesel::RunQueryDsl;

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::schema::comment;
use crate::models::comment::comment::Comment;

pub async fn delete_comment(req: HttpRequest, token: JwtToken, db: DB) -> impl Responder {
    let comment_id = req.match_info().get("comment_id").unwrap().to_string();
    let comment_id = comment_id.parse::<i32>().unwrap();

    let comment_result = comment::table
        .filter(comment::columns::user_id.eq(token.user_id.clone()))
        .filter(comment::columns::id.eq(comment_id))
        .first::<Comment>(&db.connection);

    match comment_result {
        Ok(res) => {
            let delete_result = diesel::delete(&res).execute(&db.connection);
            match delete_result {
                Ok(_) => return HttpResponse::Ok(),
                Err(_) => return HttpResponse::InternalServerError()
            }
        },
        Err(_) => return HttpResponse::NotFound().into()
    }
}