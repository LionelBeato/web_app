
use rocket::serde::Serialize; 

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Student {
    pub active: bool,
    pub first_name: String, 
    pub last_name: String,
}