use bcrypt::{DEFAULT_COST, hash};
use uuid::Uuid;
use crate::schema::users;

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}
impl NewUser {
    pub fn new(username: String, email: String, password: String) -> NewUser {
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        let unique_id = Uuid::new_v4().to_string();

        return NewUser { username, email, unique_id, password: hashed_password };
    }
}
