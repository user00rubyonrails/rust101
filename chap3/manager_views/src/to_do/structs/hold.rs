use super::base::Base;

pub struct Hold {
    pub super_struct: Base,
}

impl Hold {
    pub fn new(input_title: &str) -> Hold {
        let base: Base = Base::new(input_title, "hold");
        return Hold { super_struct: base };
    }
}
