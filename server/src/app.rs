use actix_web;

pub struct App {
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn listen(&self) -> std::io::Result<()> {
        actix_web::HttpServer::new(|| actix_web::App::new().service(ping))
            .bind(("127.0.0.1", self.port))?
            .run()
            .await
    }
}

#[actix_web::get("/ping")]
async fn ping() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("pong")
}
