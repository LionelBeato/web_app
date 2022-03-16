use std::borrow::Cow;

use rocket::serde::{Serialize, Deserialize}; 

pub type Id = usize; 

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Student<'r> {
    pub id: Option<Id>, 
    pub active: bool,
    pub first_name: Cow<'r, str>, 
    pub last_name: String,
}