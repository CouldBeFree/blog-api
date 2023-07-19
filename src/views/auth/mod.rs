mod login;
mod logout;
mod register;

use actix_web::web::{ServiceConfig, post, scope};

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .route("login", post().to(login::login))
            .route("logout", post().to(logout::logout))
            .route("register", post().to(register::register))
    );
}
