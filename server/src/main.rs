/*use server::filesystem::Filesystem;

fn main() {
    let fs = Filesystem::new("/home/danilo/Downloads".to_string(), "/home/danilo/temp_target".to_string());
    match fs.move_game("Crazy Taxi.iso".to_string()) {
        Ok(_) => println!("moved!"),
        Err(err) => println!("failed: {err}"),
    }
}*/

/*use server::common::http_client::HttpClient;
use server::core::game_download_provider::GameDownloadProvider;
use server::core::game_download_provider::crocdb::CrocdbDownloadProvider;

#[actix_web::main]
async fn main() -> () {
    let provider = CrocdbDownloadProvider::new();
    let results = provider.list("Crazy Taxi").await;
    println!("{:#?}", results);
    let destination_path = format!("/home/danilo/Downloads/{}", results[0].filename);
    let _ = HttpClient::download(&results[0].url, &destination_path).await;
}*/

use server::app::App;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = App::new(8080);
    app.listen().await
}
