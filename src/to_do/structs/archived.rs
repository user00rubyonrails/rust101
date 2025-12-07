use super::base::Base;

use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;
#[derive(Debug)]
pub struct Archived {
    pub super_struct: Base,
}

impl Archived {
    pub fn new(input_title: &str) -> Archived {
        let base: Base = Base::new(input_title, "archived");
        return Archived { super_struct: base };
    }
}

impl Get for Archived {}
impl Edit for Archived {}
impl Create for Archived {}
impl Delete for Archived {}
