pub mod create_post;
pub mod delete_post;
pub mod edit_post;
pub mod get_post;
pub mod get_posts_by_user;
pub mod create_like;

use actix_web::web::{ServiceConfig, post, scope, delete, put, get};

use self::create_post::create_post;
use self::delete_post::delete_post;
use self::edit_post::edit_post;
use self::get_post::get_post;
use self::get_posts_by_user::get_posts_by_user;
use self::create_like::create_like;

pub fn post_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/post")
        .route("", post().to(create_post))
        .route("{id}", delete().to(delete_post))
        .route("{id}", put().to(edit_post))
        .route("mine", get().to(get_posts_by_user))
        .route("{id}", get().to(get_post))
        .route("{id}/like/add", post().to(create_like))
        .route("{id}/like/update", put().to(create_like))
        .route("{id}/like/remove", delete().to(create_like))
    );
}