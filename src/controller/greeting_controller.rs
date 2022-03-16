use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::sync::Mutex;
use rocket::State;
use crate::model::Student as Student;
use crate::model::Id as Id; 

use rocket::serde::json::{Json, Value, json};

type StudentList = Mutex<Vec<String>>; 
type Students<'r> = &'r State<StudentList>; 

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

// #[get("/student")]
// pub fn get_student() -> Json<Student> {
//     Json(Student {
//              active: true,
//              first_name: String::from("Billy"),
//              last_name: String::from("Blimps")
//         }
//     )
// }

#[get("/<id>", format ="json")]
async fn get(id: Id, list:Students<'_>) -> Option<Json<Student<'_>>> {
    let list = list.lock().await;

    Some(Json(Student {
        id: Some(id),
        active: true,
        first_name: list.get(id)?.to_string().into(),
        last_name: String::from("last name lol"), 
    }))
    
}

#[post("/", format = "json", data = "<student>")]
async fn new(student: Json<Student<'_>>, list: Students<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    list.push(student.first_name.to_string()); 
    json!({"status": "ok", "id": id})
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/student", routes![new, get])
        .manage(StudentList::new(vec![]))
    })
}