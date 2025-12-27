use crate::views::to_do::utils::return_state;
use actix_web::Responder;

pub async fn get() -> impl Responder {
    return return_state();
}
