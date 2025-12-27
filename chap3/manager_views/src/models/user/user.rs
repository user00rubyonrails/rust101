use bcrypt::verify;

use crate::schema::users;

#[derive(Queryable, Clone, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    pub fn verify(self, password: String) -> bool {
        return verify(password, &self.password).unwrap();
    }
}
