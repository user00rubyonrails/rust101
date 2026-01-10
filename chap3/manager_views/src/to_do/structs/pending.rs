use super::base::Base;

#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base: Base = Base::new(input_title, "pending");
        return Pending { super_struct: base };
    }
}

#[cfg(test)]
mod pending_tests {
    use super::Pending;

    #[test]
    fn new() {
        let expected_status = String::from("pending");
        let input_title = String::from("excel date");
        let expected_title = String::from("excel date");
        let pending: Pending = Pending::new(&input_title);

        assert_eq!(expected_title, pending.super_struct.title);
        assert_eq!(expected_status, pending.super_struct.status);
    }
}
