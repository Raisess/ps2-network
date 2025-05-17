use std::time::Duration;

use server::app::App;
use server::common::http_client::HttpClient;
use server::core::filesystem::Filesystem;
use server::core::queue::queue;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::rt::spawn(async {
        loop {
            actix_web::rt::time::sleep(Duration::from_millis(1000)).await;
            let game = queue().lock().unwrap().pop_front();

            match game {
                Some(game) => {
                    println!("game: {:#?}", game);

                    let destination_path = format!("/home/danilo/Downloads/{}", game.filename);
                    let _ = HttpClient::download(&game.url, &destination_path).await;
                    // @TODO: unzip downloaded file
                    // @TODO: rename the file to GAME_ID.name.iso pattern

                    // @TODO: validate fs path's
                    let fs = Filesystem::new(
                        "/home/danilo/Downloads".to_string(),
                        "/home/danilo/temp_target".to_string(),
                    );
                    match fs.move_game(game.filename) {
                        Ok(_) => {}
                        Err(err) => panic!("Failed to move game {err}"),
                    }

                    // @TODO: download game art
                    // should do that directly on the target directory???
                }
                None => {}
            }
        }
    });

    let app = App::new(8080);
    app.listen().await
}
