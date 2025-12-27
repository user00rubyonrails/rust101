use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;
use crate::views::to_do::utils::return_state;
use actix_web::{HttpRequest, Responder};
use diesel::prelude::*;

pub async fn create(req: HttpRequest) -> impl Responder {
    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_reference: String = title.clone();

    let conn = establish_connection();
    // filter if title exited in db before create
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_reference))
        .order(to_do::columns::id.asc())
        .load::<Item>(&conn)
        .unwrap();
    if items.len() == 0 {
        let new_post = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&conn);
    }

    return return_state();
}
