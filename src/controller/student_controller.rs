use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::sync::Mutex;
use rocket::State;
use crate::model::Student as Student;
use crate::model::Id as Id;
use tokio_stream::StreamExt;
use std::collections::HashMap;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::types::SdkError;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};


use rocket::serde::json::{Json, Value, json};

type StudentList = Mutex<Vec<Student>>; 
// type Students<'r> = &'r State<StudentList>; 
type Students<'r> = &'r State<StudentList>; 

async fn get_database() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config); // passing config by reference
    Ok(client) 
}

async fn all_items(client: &Client, table: &str) -> Result<Vec<Student>, Error> {
    println!("inside all_items");
    let items: Result<Vec<_>, _> = client
        .scan()
        .table_name(table)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

    let mut student_vec: Vec<Student> = Vec::new();


    for item in items? {

        let process = |name:&str| item.get(name)
            .unwrap()
            .as_s()
            .unwrap();
        
        let active = item.get("active")
            .unwrap()
            .as_bool()
            .unwrap(); 
        
        let first_name = process("first_name");
        let id = process("id");
        let last_name = process("last_name");

        student_vec.push(
                Student { 
                    id: Some(str::parse(id).unwrap()), 
                    active: active.clone(), 
                    first_name: first_name.clone(), 
                    last_name: last_name.clone(),
                });
    }

    Ok(student_vec)
}

#[get("/")]
pub async fn index() -> Option<Value> {
    
    let client = get_database().await.unwrap();
    let students = all_items(&client, "student").await.ok()?; 

    Some(json!(students))
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/<id>", format = "json")]
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