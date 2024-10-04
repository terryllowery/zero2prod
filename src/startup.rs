use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use actix_web::middleware::Logger;

// TODO: Wrap this two into one use statement
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;




pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
   let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
         App::new()
         .wrap(Logger::default())
         .route("/healthcheck", web::get().to(health_check))
         .route("/subscriptions", web::post().to(subscribe))
         .app_data(connection.clone())
     })
         .listen(listener)?
         .run();
     Ok(server)
 }