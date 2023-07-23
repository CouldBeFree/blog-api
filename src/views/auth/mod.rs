mod login;
mod logout;
mod register;
mod get_me;

use actix_web::web::{ServiceConfig, post, scope, get};

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .route("login", post().to(login::login))
            .route("logout", post().to(logout::logout))
            .route("register", post().to(register::register))
            .route("get_me", get().to(get_me::get_me))
    );
}
