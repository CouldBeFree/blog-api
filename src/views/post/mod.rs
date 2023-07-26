pub mod create_post;
pub mod delete_post;
pub mod edit_post;
pub mod get_post;
pub mod get_posts_by_user;
pub mod create_like;
pub mod update_like;
pub mod remove_like;
pub mod create_comment;
pub mod edit_comment;
pub mod delete_comment;

use actix_web::web::{ServiceConfig, post, scope, delete, put, get};

use self::create_post::create_post;
use self::delete_post::delete_post;
use self::edit_post::edit_post;
use self::get_post::get_post;
use self::get_posts_by_user::get_posts_by_user;
use self::create_like::create_like;
use self::update_like::update_like;
use self::remove_like::remove_like;
use self::create_comment::create_comment;
use self::edit_comment::edit_comment;
use self::delete_comment::delete_comment;

pub fn post_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/post")
        .route("", post().to(create_post))
        .route("{id}", delete().to(delete_post))
        .route("{id}", put().to(edit_post))
        .route("mine", get().to(get_posts_by_user))
        .route("{id}", get().to(get_post))
        .route("{id}/like/add", post().to(create_like))
        .route("{id}/like/update", put().to(update_like))
        .route("{id}/like/remove", delete().to(remove_like))
        .route("{id}/comment/add", post().to(create_comment))
        .route("{id}/comment/{comment_id}/update", put().to(edit_comment))
        .route("comment/{comment_id}", delete().to(delete_comment))
    );
}