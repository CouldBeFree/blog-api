use crate::diesel;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{Responder, HttpResponse, HttpRequest};

use crate::models::post::post::Post;
use crate::json_serialization::post::Post as Json_Response;
use crate::database::DB;
use crate::schema::post;

pub async fn get_post(req: HttpRequest, db: DB) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().to_string();
    let integer: i32 = id.parse::<i32>().unwrap();
    let post = post::table
        .filter(post::columns::id.eq(integer))
        .first::<Post>(&db.connection);

    match post {
        Ok(res) => {
            let json = Json_Response {
                id: Some(res.id),
                title: res.title,
                content: res.content,
                date: Some(res.date),
            };
            return HttpResponse::Ok().json(json);
        }
        Err(_) => return HttpResponse::NotFound().into()
    }
}