use std::sync::{Arc, Mutex};
use crate::ws::{BroadcastMessage};
use actix::Addr;
use crate::ws::WsServer;
use actix_web::{
    delete, get, post,
    web::{Data, Json},
    HttpResponse, Responder,
    
};


use crate::{
    orderbook::Orderbook,
    types::{CreateOrder, DeleteOrder},
};

#[get("/depth")]
pub async fn get_depth(
    orderbook: Data<Arc<Mutex<Orderbook>>>,
) -> impl Responder {
    let ob = orderbook.lock().unwrap();
    let depth = ob.get_depth();
    HttpResponse::Ok().json(depth)
}



#[post("/order")]
pub async fn create_order(
    orderbook: Data<Arc<Mutex<Orderbook>>>,
    server: Data<Addr<WsServer>>,
    order: Json<CreateOrder>,
) -> impl Responder {
    
    let mut ob = orderbook.lock().unwrap();
    let report = ob.create_order(order.0);

    let depth = ob.get_depth();
    let ticker = ob.get_ticker();
    let trades = ob.get_trades();

    let payload = serde_json::json!({
        "type": "market_update",
        "depth": depth,
        "ticker": {
            "best_bid": ticker.0,
            "best_ask": ticker.1
        },
        "trades": trades
    });

    server.do_send(BroadcastMessage(payload.to_string()));

    HttpResponse::Ok().json(report)
}


#[delete("/order")]
pub async fn delete_order(
    orderbook: Data<Arc<Mutex<Orderbook>>>,
    order: Json<DeleteOrder>,
) -> impl Responder {
    let mut ob = orderbook.lock().unwrap();
    ob.delete_order(order.0);
    HttpResponse::Ok().body("Order Deleted")
}

#[get("/ticker")]
pub async fn get_ticker(
    orderbook: Data<Arc<Mutex<Orderbook>>>,
) -> impl Responder {
    let ob = orderbook.lock().unwrap();
    let (best_bid, best_ask) = ob.get_ticker();

    HttpResponse::Ok().json(serde_json::json!({
        "best_bid": best_bid,
        "best_ask": best_ask
    }))
}

#[post("/reset")]
pub async fn reset(orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    let mut ob = orderbook.lock().unwrap();
    ob.clear();
    HttpResponse::Ok().body("Orderbook cleared")
}
