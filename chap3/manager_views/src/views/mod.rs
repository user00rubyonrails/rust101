use actix_web::web;

mod auth;
mod path;
mod to_do;
mod user;
mod app;
pub mod token;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
    user::user_factory(app);
    app::app_factory(app);
}
