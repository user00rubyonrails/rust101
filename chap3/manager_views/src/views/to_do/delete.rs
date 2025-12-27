use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::{diesel, schema::to_do};
use diesel::prelude::*;

use actix_web::{HttpResponse, web};

use super::utils::return_state;
use crate::json_serialization::to_to_item::ToDoItem; // use super:: - use from /current  Dir ()

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_reference: &String = &to_do_item.title.clone();
    let conn = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(title_reference))
        .order(to_do::columns::id.asc())
        .load::<Item>(&conn)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&conn);
    return HttpResponse::Ok().json(return_state());
}

// use actix_web::{HttpResponse, web};
// use serde_json::Map;
// use serde_json::value::Value;

// use crate::json_serialization::to_to_item::ToDoItem;
// use crate::processes::process_input;
// use crate::state::read_file;
// use crate::to_do::to_do_factory; // use crate::  - use from /root (or /src)
// use super::utils::return_state; // use super:: - use from /current  Dir ()

// pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
//     let state: Map<String, Value> = read_file(String::from("./state.json"));
//     let item_title = to_do_item.title.clone();
//     let status = to_do_item.status.clone();

//     match to_do_factory(&status, &item_title) {
//         Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
//         Ok(item) => process_input(item, String::from("delete"), &state),
//     }

//     return HttpResponse::Ok().json(return_state())
// }
