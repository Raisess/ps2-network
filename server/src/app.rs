use actix_web;

use crate::handler::list_games::ListGamesHandler;
use crate::handler::Handler;

pub struct App {
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn listen(&self) -> std::io::Result<()> {
        actix_web::HttpServer::new(|| {
            actix_web::App::new()
                .service(ping)
                .service(search)
                .service(list_downloads)
                .service(download)
        })
        .bind(("0.0.0.0", self.port))?
        .run()
        .await
    }
}

#[actix_web::get("/ping")]
async fn ping() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("pong")
}

#[derive(serde::Deserialize)]
struct Search {
    key: String,
}

#[actix_web::get("/search")]
async fn search(query: actix_web::web::Query<Search>) -> impl actix_web::Responder {
    let handler = ListGamesHandler {
        search_key: query.key.clone(),
    };

    let response = handler.handle().await;
    actix_web::HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
}

#[actix_web::get("/downloads")]
async fn list_downloads() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("TODO")
}

#[actix_web::post("/download")]
async fn download() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("TODO")
}
