use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use diesel::RunQueryDsl;

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::schema::post;
use crate::schema::like;
use crate::models::post::post::Post as Post_Model;
use crate::json_serialization::like::Like as Json_Like;
use crate::models::post::new_like::NewLike;
use crate::models::post::like::Like;

pub async fn create_like(req: HttpRequest, like: web::Json<Json_Like>, token: JwtToken, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();
    let parsed_id = id.parse::<i32>().unwrap();
    let post = post::table
        .filter(post::columns::id.eq(parsed_id))
        .get_result::<Post_Model>(&db.connection);

    match post {
        Ok(_) => {
            let like = NewLike::new(
                like.reaction.clone(),
                parsed_id.clone(),
                token.user_id.clone()
            );
            let already_exists_like = like::table
                .filter(like::columns::user_id.eq(token.user_id))
                .filter(like::columns::post_id.eq(parsed_id))
                .first::<Like>(&db.connection);

            match already_exists_like {
                Ok(_) => {
                    return HttpResponse::NotAcceptable()
                },
                Err(_) => {
                    let _ = diesel::insert_into(like::table)
                        .values(&like)
                        .load::<Like>(&db.connection)
                        .unwrap();
                    return HttpResponse::Created()
                }
            }

            
        },
        Err(_) => return HttpResponse::NotFound()
    }
}