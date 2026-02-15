use actix::{
    Actor, Addr, AsyncContext, Context, Handler, Message, Recipient, StreamHandler,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::collections::HashSet;

//
// ============ SERVER ACTOR ============
//

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
struct Connect {
    addr: Recipient<BroadcastMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    _id: usize,
}

pub struct WsServer {
    sessions: HashSet<Recipient<BroadcastMessage>>,
}

impl WsServer {
    pub fn new() -> Self {
        Self {
            sessions: HashSet::new(),
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(msg.addr);
        self.sessions.len()
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

fn handle(&mut self, _msg: Disconnect, _: &mut Context<Self>) {
        // No-op for now
    }
}

impl Handler<BroadcastMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        for session in &self.sessions {
            let _ = session.do_send(BroadcastMessage(msg.0.clone()));
        }
    }
}

//
// ============ SESSION ACTOR ============
//

pub struct WsSession {
    pub server: Addr<WsServer>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.server.do_send(Connect {
            addr: addr.recipient(),
        });
    }
}

impl Handler<BroadcastMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => {}
        }
    }
}

//
// ============ ROUTE ============
//

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            server: server.get_ref().clone(),
        },
        &req,
        stream,
    )
}
