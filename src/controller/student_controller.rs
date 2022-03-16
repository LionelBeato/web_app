use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::sync::Mutex;
use rocket::State;
use crate::model::Student as Student;
use crate::model::Id as Id;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};


use rocket::serde::json::{Json, Value, json};

const MY_CONST: u32 = 0; 

type StudentList = Mutex<Vec<Student>>; 
// type Students<'r> = &'r State<StudentList>; 
type Students<'r> = &'r State<StudentList>; 

async fn get_database() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config); // passing config by reference
    let resp = client.list_tables().send().await?;
    
    println!("Tables:"); 

    let names = resp.table_names().unwrap_or_default(); 

    for name in names {
        println!(" {}", name); 
    }

    println!("Found {} tables", names.len());
    Ok(resp) 
}

#[get("/")]
pub async fn index() -> Option<String> {
    get_database().await;
    Some("wow!".to_string())
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/<id>", format ="json")]
async fn get(id: Id, list:Students<'_>) -> Option<Json<Student>> {

    let list = list.lock().await; 

    Some(Json(Student {
        id: Some(id),
        active: list.get(id)?.active,
        first_name: list.get(id)?.first_name.to_string(),
        last_name: list.get(id)?.last_name.to_string(),
    }))
}

#[post("/", format = "json", data = "<student>")]
async fn new(student: Json<Student>, list: Students<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();

    list.push(student.into_inner()); 
    json!({"status": "ok", "id": id})
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resources was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/student", routes![new, get])
        .register("/student", catchers![not_found])
        .manage(StudentList::new(vec![]))
    })
}