use serde_json::Map;
use serde_json::value::Value;

use crate::{json_serialization::to_to_items::ToDoItems, state::read_file, to_do::to_do_factory};

pub fn return_state() -> ToDoItems {
    let state: Map<String, Value> = read_file(String::from("./state.json"));

    let mut input_items = Vec::new();
    for (key, value) in state {
        let item_type = String::from(value.as_str().unwrap());

        let item = to_do_factory(&item_type, &key).unwrap();
        input_items.push(item);
    }

    return ToDoItems::new(input_items)
}
