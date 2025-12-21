use actix_web::{HttpResponse, web};
use serde_json::Map;
use serde_json::value::Value;

use crate::json_serialization::to_to_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory; // use crate::  - use from /root (or /src)
use super::utils::return_state; // use super:: - use from /current  Dir ()

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title_reference:&String = &to_do_item.title.clone();
    let item_title = to_do_item.title.clone();

    let status: String;
    match &state.get(title_reference) {
        Some(res) => {
            status = res.to_string().replace('\"', "");
        },
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", title_reference))
        }
    }

    if &status == &to_do_item.status {
        return HttpResponse::Ok().json(return_state())
    }

    match to_do_factory(&status, &item_title) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("edit"), &state),
    }

    return HttpResponse::Ok().json(return_state())
}
