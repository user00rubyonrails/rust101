#[macro_use]
extern crate diesel; // extern: ben ngoai-
extern crate dotenv;

use actix_service::Service;
use actix_web::{App, HttpResponse, HttpServer};
use futures::future::{Either, ok};

use log;
use env_logger;

mod views;

mod database;
mod json_serialization;
mod models;
mod schema;
mod to_do;

// Authenticating our users
mod auth;

// used to include mod for `rust-alayzer` working on file
// #[path = "views/to_do/utils.rs"]
// mod test;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, app_routing| {
                let request_url: String = String::from(*&req.uri().path());
                let passed: bool;
                // .wrap_fn(|req, app_routing| {}) - this configurates the `middleware` for the server via a closure
                // inside the middleware which is check token have 3 cases:
                // 1. NO token in headers
                // 2. WRONG token in headers
                // 3. RIGHT token in headers
                if *&req.path().contains("/api/v1/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => {
                            passed = true;
                        }
                        Err(_msg) => {
                            passed = false;
                        }
                    }
                } else {
                    passed = true;
                }
                // create fut async/await belong to the routing, then await to complete and return result
                let end_result = match passed {
                    true => {
                        Either::Left(app_routing.call(req))
                    },
                    false => {
                        Either::Right(ok(
                        req.into_response(HttpResponse::Unauthorized().finish().into_body())
                    ))},
                };
                async move {
                    let result = end_result.await?;
                    log::info!("{} -> {}", request_url, &result.status());
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .workers(4)
    .run()
    .await
}
