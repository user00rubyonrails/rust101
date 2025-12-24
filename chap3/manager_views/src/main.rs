use actix_service::Service;
use actix_web::{App, HttpServer};
mod views;

mod json_serialization;
mod processes;
mod state;
mod to_do;

// used to include mod for `rust-alayzer` working on file
// #[path = "app/items.rs"]
// mod test;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, app_routing| {
                // .wrap_fn(|req, app_routing| {}) - this configurates the `middleware` for the server via a closure
                // inside the middleware which is check token have 3 cases:
                // 1. NO token in headers
                // 2. WRONG token in headers
                // 3. RIGHT token in headers
                if *&req.path().contains("/item") {
                    match views::token::process_token(&req) {
                        Ok(_token) => println!("the token is passable!"),
                        Err(msg) => println!("token error: {}", msg),
                    }
                }
                // create fut async/await belong to the routing, then await to complete and return result
                let fut = app_routing.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
