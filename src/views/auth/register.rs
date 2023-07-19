use crate::diesel;
use diesel::prelude::*;
use validator::Validate;
use serde_json::json;
use diesel::result::Error;
use bcrypt::{DEFAULT_COST, hash};

use actix_web::{web, Responder, HttpResponse};

use crate::database::DB;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::user_table;

fn check_email(email: &str, db: &DB) -> Result<usize, Error> {
    let email = user_table::table
        .filter(user_table::columns::email.eq(&email))
        .execute(&db.connection);

    match email {
        Ok(res) => Ok(res),
        Err(err) => Err(err)
    }
}

fn check_username(username: &str, db: &DB) -> Result<usize, Error> {
    let email = user_table::table
        .filter(user_table::columns::username.eq(&username))
        .execute(&db.connection);

    match email {
        Ok(res) => Ok(res),
        Err(err) => Err(err)
    }
}

pub async fn register(new_user: web::Json<NewUserSchema>, db: DB) -> impl Responder {
    let validation_result = new_user.validate();

    if validation_result.is_err() {
        let errors = validation_result.unwrap_err();
        return HttpResponse::BadRequest().json(errors)
    }

    let hashed_password: String = hash(new_user.password.as_str(), DEFAULT_COST).unwrap();

    let new_user = NewUser::new(
        new_user.username.clone(),
        new_user.email.clone(),
        hashed_password
    );

    let user_result = check_username(&new_user.username, &db);

    match user_result {
        Ok(res) => {
            if res != 0 {
                let json_response = json!({"error": "Username already exists"});
                return HttpResponse::BadRequest().json(json_response)
            }
        },
        Err(_) => return HttpResponse::Conflict().finish()
    }

    let email_result = check_email(&new_user.email, &db);

    match email_result {
        Ok(res) => {
            if res != 0 {
                let json_response = json!({"error": "Email already exists"});
                return HttpResponse::BadRequest().json(json_response)
            }
        },
        Err(_) => return HttpResponse::Conflict().finish()
    }

    let insert_result = 
        diesel::insert_into(user_table::table)
            .values(&new_user)
            .execute(&db.connection);

    match insert_result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::Conflict().finish()
    }
}