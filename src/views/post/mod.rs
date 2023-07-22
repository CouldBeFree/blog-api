pub mod create_post;
pub mod delete_post;
pub mod edit_post;
pub mod get_post;

use actix_web::web::{ServiceConfig, post, scope, delete, put, get};

use self::create_post::create_post;
use self::delete_post::delete_post;
use self::edit_post::edit_post;
use self::get_post::get_post;

pub fn post_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/post")
        .route("create", post().to(create_post))
        .route("delete/{id}", delete().to(delete_post))
        .route("update/{id}", put().to(edit_post))
        .route("get/{id}", get().to(get_post))
    );
}