pub trait Delete {
    fn delete(&self, title: &str) {
        println!("{} id being deleted", title);
    }
}
