use crate::diesel;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{Responder, HttpResponse, HttpRequest};

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::models::post::post::Post;
use crate::schema::post;

pub async fn delete_post(req: HttpRequest, token: JwtToken, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();
    let post = post::table
        .filter(post::columns::user_id.eq(token.user_id.clone()))
        .filter(post::columns::id.eq(id.parse::<i32>().unwrap()))
        .first::<Post>(&db.connection);

    match post {
        Ok(res) => {
            let delete_result = diesel::delete(&res).execute(&db.connection);
            match delete_result {
                Ok(_) => return HttpResponse::Ok(),
                Err(_) => return HttpResponse::InternalServerError()
            }
        }
        Err(_) => return HttpResponse::NotFound()
    }
}