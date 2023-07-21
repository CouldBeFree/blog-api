pub mod create_post;
pub mod delete_post;

use actix_web::web::{ServiceConfig, post, scope, delete};

use self::create_post::create_post;
use self::delete_post::delete_post;

pub fn post_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/post")
        .route("create", post().to(create_post))
        .route("delete/{id}", delete().to(delete_post))
    );
}