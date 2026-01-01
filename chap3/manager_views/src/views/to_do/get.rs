use crate::{auth::jwt::JwtToken, views::to_do::utils::return_state};
use actix_web::{HttpRequest, Responder};

pub async fn get(request: HttpRequest) -> impl Responder {
    let token = JwtToken::decode_from_request(request).unwrap();
    return return_state(&token.user_id);
}
