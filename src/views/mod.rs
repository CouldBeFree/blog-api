mod auth;
mod post;

use auth::auth_views_factory;
use post::post_views_factory;

use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    post_views_factory(app);
}
