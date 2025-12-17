use actix_web::{App, HttpServer};
mod views;

mod state;
mod to_do;
mod processes;

// #[path = "views/to_do/mod.rs"]
// mod test;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app
    }).bind("127.0.0.1:8000")?
    .run()
    .await
}
