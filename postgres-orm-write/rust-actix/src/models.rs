use serde_derive::Serialize;
use super::schema::users;

#[derive(Serialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_enabled: bool
}

#[derive(Serialize , Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub is_enabled: bool,

}
