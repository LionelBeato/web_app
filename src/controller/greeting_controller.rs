use rocket::tokio::time::{sleep, Duration};
use crate::model::Student as Student; 

use rocket::serde::json::Json;


#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/student")]
pub fn get_student() -> Json<Student> {
    Json(Student {
             active: true,
             first_name: String::from("Billy"),
             last_name: String::from("Blimps")
        }
    )
}
