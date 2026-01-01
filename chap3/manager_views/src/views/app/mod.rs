mod items;
mod login;
mod logout;
mod content_loader;
use super::path::Path;
use actix_web::web;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix: String::from("/"), backend: true};
    app.route(&base_path.define(String::from("")), web::get().to(items::items));
    app.route(&base_path.define(String::from("login")), web::get().to(login::login));
    app.route(&base_path.define(String::from("logout")), web::get().to(logout::logout));
}
