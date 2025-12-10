use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(tcp_listener: TcpListener, db_connecton_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_connecton_pool = web::Data::new(db_connecton_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_connecton_pool.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
