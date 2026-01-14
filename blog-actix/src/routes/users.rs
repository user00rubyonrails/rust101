use crate::errors::AppError;
use crate::routes::convert;
use crate::{Pool, models};
use actix_web::{HttpResponse, web};
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

fn create_user(
    item: web::Json<UserInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let username = item.into_inner().username;
        models::create_user(conn, username.as_str())
    })
    .then(convert)
}

fn find_user(
    name: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let name = name.into_inner();
        let key = models::UserKey::Username(name.as_str());
        models::find_user(conn, key)
    })
    .then(convert)
}

fn get_user(
    id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let id = id.into_inner();
        let key = models::UserKey::ID(id);
        models::find_user(conn, key)
    })
    .then(convert)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::post().to_async(create_user)))
        .service(web::resource("/users/find/{name}").route(web::get().to_async(find_user)))
        .service(web::resource("/users/{id}").route(web::get().to_async(get_user)));
}
