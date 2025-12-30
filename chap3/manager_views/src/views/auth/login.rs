use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use crate::{auth::jwt::JwtToken, database::establish_connection, json_serialization::login::Login, models::user::user::User, schema::users};

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    println!("[LOG] pub async fn login");
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let conn = establish_connection();
    let users = users::table
        .filter(users::columns::username.eq(username.as_str()))
        .load::<User>(&conn).unwrap();
    if users.len() == 0 { // no user in db: notfound!
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 { // have more than 1 user in db: conflict!!
        return HttpResponse::Conflict().await.unwrap()
    }

    match users[0].clone().verify(password) { // get the first index 0 user and return `token` header
        true => {
            let token = JwtToken::encode(users[0].clone().id);
            HttpResponse::Ok().header("token", token).await.unwrap()
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }
}
