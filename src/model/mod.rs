use rocket::serde::{Serialize, Deserialize}; 

pub type Id = usize; 

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Student {
    pub id: Option<Id>, 
    pub active: bool,
    pub first_name: String, 
    pub last_name: String,
}