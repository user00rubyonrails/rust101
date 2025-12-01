mod todo;

use todo::structs::done::Done;
use todo::structs::pending::Pending;
fn main() {
    let done: Done = Done::new("shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending: Pending = Pending::new("laundry");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);
}
