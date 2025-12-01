mod to_do;

use to_do::ItemTypes;
use to_do::to_do_factory;
fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");
    match to_do_item.unwrap() {
        ItemTypes::Done(i) => println!("it's a done item w the title: {}", i.super_struct.title),
        ItemTypes::Pending(i) => {
            println!("it's a pending item w the title: {}", i.super_struct.title)
        }
    }
}
