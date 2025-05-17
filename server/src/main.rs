use server::app::App;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = App::new(8080);
    app.listen().await
}
