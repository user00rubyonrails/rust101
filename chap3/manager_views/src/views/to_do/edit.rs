use crate::database::establish_connection;
use crate::{diesel, schema::to_do};
use diesel::prelude::*;

use actix_web::{HttpResponse, web};

use super::utils::return_state;
use crate::json_serialization::to_to_item::ToDoItem; // use super:: - use from /current  Dir ()

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_reference: &String = &to_do_item.title.clone();
    let conn = establish_connection();

    let results = to_do::table.filter(to_do::columns::title.eq(title_reference));
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&conn);

    return HttpResponse::Ok().json(return_state());
}
