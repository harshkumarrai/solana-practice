use std::sync::{Arc, Mutex};

use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use crate::orderbook::Orderbook;
use ws::WsServer;

pub mod routes;
pub mod types;
pub mod orderbook;
pub mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orderbook = Arc::new(Mutex::new(Orderbook::default()));
let port = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string());



    let ws_server = WsServer::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(orderbook.clone()))
            .app_data(web::Data::new(ws_server.clone()))
            .service(routes::create_order)
            .service(routes::delete_order)
            .service(routes::get_depth)
            .service(routes::get_ticker)
            .service(routes::reset)
            .route("/ws", web::get().to(ws::ws_route))
    })
  .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
