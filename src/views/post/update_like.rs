use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use diesel::RunQueryDsl;

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::schema::like;
use crate::json_serialization::like::Like as Json_Like;
use crate::models::post::like::Like;

pub async fn update_like(req: HttpRequest, like: web::Json<Json_Like>, token: JwtToken, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();
    let parsed_id = id.parse::<i32>().unwrap();
    let existing_like = like::table
        .filter(like::columns::post_id.eq(parsed_id))
        .filter(like::columns::user_id.eq(token.user_id))
        .first::<Like>(&db.connection);

    match existing_like {
        Ok(_) => {
            let update_result = diesel::update(
                like::table
                .filter(like::columns::post_id.eq(parsed_id))
                .filter(like::columns::user_id.eq(token.user_id))
            )
            .set(like::columns::reaction.eq(&like.reaction))
            .load::<Like>(&db.connection);

            match update_result {
                Ok(_) => return HttpResponse::Ok(),
                Err(_) => return HttpResponse::NotFound()
            }
        },
        Err(_) => return HttpResponse::NotFound()
    }
}