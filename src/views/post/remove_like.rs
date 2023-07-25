use crate::diesel;
use diesel::prelude::*;
use actix_web::{Responder, HttpResponse, HttpRequest};
use diesel::RunQueryDsl;

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::schema::like;
use crate::models::post::like::Like;

pub async fn remove_like(req: HttpRequest, token: JwtToken, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();
    let parsed_id = id.parse::<i32>().unwrap();
    let like_result = like::table
        .filter(like::columns::user_id.eq(token.user_id.clone()))
        .filter(like::columns::post_id.eq(parsed_id))
        .first::<Like>(&db.connection);

    match like_result {
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