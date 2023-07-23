use crate::diesel;
use diesel::prelude::*;
use actix_web::{HttpResponse, Responder};

use crate::jwt::JwtToken;
use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::user::User as Json_User;
use crate::schema::user_table;

pub async fn get_me(token: JwtToken, db: DB) -> impl Responder {
    let user = user_table::table
        .filter(user_table::columns::id.eq(token.user_id))
        .first::<User>(&db.connection);

    match user {
        Ok(res) => {
            let json = Json_User {
                id: res.id,
                email: res.email,
                username: res.username,
            };
            return HttpResponse::Ok().json(json);
        },
        Err(_) => return HttpResponse::NotFound().into()
    }

}