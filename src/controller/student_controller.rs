use rocket::futures::TryFutureExt;
// use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::sync::Mutex;
use rocket::State;
use crate::model::Student as Student;
use crate::model::Id as Id;
use tokio_stream::StreamExt;
use std::collections::HashMap;
use aws_sdk_dynamodb::model::{AttributeValue, Select};
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};


use rocket::serde::json::{Json, Value, json};

// type StudentList = Mutex<Vec<Student>>; 
// type Students<'r> = &'r State<StudentList>; 

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

#[get("/hello?wave&<name>")]
fn hello(name: Option<String>) -> String {
    name.map(|name| format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hello!".into())
}

#[get("/?<id>&<first_name>&<last_name>")]
async fn getByQuery(id: Id, first_name: String, last_name:String) -> Json<Student> {

    let client = get_database().await.unwrap();
    let q = client.query()
          .table_name("student")
          .key_condition_expression("#id = :id")
          .expression_attribute_names("#id", "id")
          .expression_attribute_values(":id", AttributeValue::S(id.to_string()))
          .select(Select::AllAttributes)
          .send()
          .await;

    let unwrapped = q.unwrap(); 
    let student = unwrapped.items().unwrap().get(0).unwrap();

    Json(Student {
        id: Some(str::parse(student.get("id").unwrap().as_s().unwrap()).unwrap()),
        active: student.get("active").unwrap().as_bool().unwrap().clone(),
        first_name: student.get("first_name").unwrap().as_s().unwrap().to_string(),
        last_name: student.get("last_name").unwrap().as_s().unwrap().to_string(),
    })
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
        rocket.mount("/student", routes![getByQuery, index, hello])
        .register("/student", catchers![not_found])
        .manage(StudentList::new(vec![]))
    })
}