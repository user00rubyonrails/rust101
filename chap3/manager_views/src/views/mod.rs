use std::env;

use actix_web::web;

mod auth;
mod path;

pub fn views_factory(app: &mut web::ServiceConfig) {
    // auth::auth_factory(app);
    let args: Vec<String> = env::args().collect();
    let params: &String = &args[args.len() - 1];

    if params.as_str() == "cancel_logout" {
        println!("logout view isn't being configured");
    } else {
        println!("logout view is being configured");
        auth::auth_factory(app, true);
    }
}
