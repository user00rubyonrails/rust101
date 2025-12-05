pub trait Create {
    fn create(&self, title: &str) {
        println!("{} id being created", title)
    }
}
