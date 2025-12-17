use super::base::Base;

use super::traits::edit::Edit;
use super::traits::get::Get;

pub struct Hold {
    pub super_struct: Base,
}

impl Hold {
    pub fn new(input_title: &str) -> Hold {
        let base: Base = Base::new(input_title, "hold");
        return Hold { super_struct: base };
    }
}

impl Get for Hold {}
impl Edit for Hold {}
