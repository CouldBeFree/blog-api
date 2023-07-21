use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::user_table;

use crate::jwt::JwtToken;

pub async fn login(credentials: web::Json<Login>, db: DB) -> impl Responder {
    let users = user_table::table
        .filter(user_table::columns::username.eq(
            credentials.username.clone()
        )).load::<User>(&db.connection)
        .unwrap();

        if users.len() == 0 {
            return HttpResponse::Unauthorized()
        }

     match users[0].clone().verify(credentials.password.clone()) {
        true => {
            let token = JwtToken::new(users[0].id);
            let raw_token = token.encode();
            HttpResponse::Ok().append_header(("token", raw_token)).take()
        },
        false => HttpResponse::Unauthorized()
     }

}