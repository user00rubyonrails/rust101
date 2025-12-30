use crate::models::user::new_user::NewUser;
use crate::schema::{users};
use crate::{database::establish_connection, json_serialization::new_user::NewUserSchema}
use actix_web::{HttpResponse, web};
use diesel::prelude::*;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
    let conn = establish_connection();

    let username = new_user.name.clone();
    let email = new_user.email.clone();
    let password = new_user.password.clone();

    let new_user = NewUser::new(username, email, password);

    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn);

    match insert_result {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::Conflict().await.unwrap(),
    }
}
