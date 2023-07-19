pub mod create_post;

use actix_web::web::{ServiceConfig, post, scope};

use self::create_post::create_post;

pub fn post_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/post")
        .route("create", post().to(create_post))
    );
}