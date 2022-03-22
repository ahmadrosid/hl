mod websocket;

use actix_web::http::StatusCode;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;

use websocket::MyWebSocket;

async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let content = std::fs::read_to_string("table.html").expect("table not found");
    let body = include_str!("index.html")
        .to_string()
        .replace("{content}", &content);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
