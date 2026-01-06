use crate::{auth::jwt::JwtToken, database::establish_connection};
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;
use crate::views::to_do::utils::return_state;
use actix_web::{HttpRequest, Responder};
use diesel::prelude::*;

pub async fn create(request: HttpRequest) -> impl Responder {
    println!("{:?}", &request);
    let title: String = request.match_info().get("title").unwrap().to_string();
    let title_reference: String = title.clone();

    let token = JwtToken::decode_from_request(request).unwrap();

    let conn = establish_connection();
    // filter if title exited in db before create
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_reference))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&conn)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&conn);
    }

    return return_state(&token.user_id);
}
