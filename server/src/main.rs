/*use server::filesystem::Filesystem;

fn main() {
    let fs = Filesystem::new("/home/danilo/Downloads".to_string(), "/home/danilo/temp_target".to_string());
    match fs.move_game("Crazy Taxi.iso".to_string()) {
        Ok(_) => println!("moved!"),
        Err(err) => println!("failed: {err}"),
    }
}*/

use server::app::App;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = App::new(8080);
    app.listen().await
}
