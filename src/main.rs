use actix_service::Service;
use actix_web::{App, HttpServer, HttpResponse, middleware::Logger};
use actix_cors::Cors;

mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // TODO: configure CORS
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        let app = App::new()
            .wrap_fn(|req, srv| {
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            }).configure(views::views_factory).wrap(cors);
            return app
    })
        .bind("127.0.0.1:5052")?
        .run()
        .await
}
