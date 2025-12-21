use actix_web::{Responder, web};
use serde_json::{Map, Value};

use crate::{json_serialization::to_to_items::ToDoItems, state::read_file, to_do::to_do_factory};

pub fn return_state() -> impl Responder {
    let state: Map<String, Value> = read_file(String::from("./state.json"));

    let mut array_buffer = Vec::new();
    for (key, value) in state {
        let item_type = String::from(value.as_str().unwrap());
        let item = to_do_factory(&item_type, &key).unwrap();

        array_buffer.push(item);
    }

    let return_pkg: ToDoItems = ToDoItems::new(array_buffer);

    return web::Json(return_pkg);
}
