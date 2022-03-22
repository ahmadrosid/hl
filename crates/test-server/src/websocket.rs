use std::time::{Duration, Instant, SystemTime};

use actix::prelude::*;
use actix_web_actors::ws;
use std::fs;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const FILE_PATH: &'static str = "table.html";

pub struct MyWebSocket {
    hb: Instant,
    modified: SystemTime,
}

impl MyWebSocket {
    pub fn new() -> Self {
        let metadata = fs::metadata(FILE_PATH).unwrap();
        let modified = metadata.modified().unwrap();

        Self {
            hb: Instant::now(),
            modified,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");

                ctx.stop();

                return;
            }

            let modified = fs::metadata(FILE_PATH).unwrap().modified().unwrap();
            if modified.duration_since(act.modified).unwrap() > Duration::from_millis(0) {
                act.modified = modified;
                println!("Sending file changes event! {}", &FILE_PATH);
                ctx.text("file_changed")
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
