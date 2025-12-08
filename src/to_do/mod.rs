pub mod structs;

// use factory pattern to manage structs
use structs::done::Done;
use structs::hold::Hold;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
    Hold(Hold),
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    if item_type == "pending" {
        let pending_item = Pending::new(item_title);
        Ok(ItemTypes::Pending(pending_item))
    } else if item_type == "done" {
        let done_item = Done::new(item_title);
        Ok(ItemTypes::Done(done_item))
    } else if item_type == "hold" {
        let hold_item = Hold::new(item_title);
        Ok(ItemTypes::Hold(hold_item))
    } else {
        Err("this is not accepted")
    }
}
