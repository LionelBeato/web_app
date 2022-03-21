use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

mod controller;
mod model;


#[macro_use]
extern crate rocket;

async fn get_database() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config); // passing config by reference
    Ok(client) 
}

#[launch]
async fn rocket() -> _ {

    let client = get_database().await.unwrap();

    rocket::build().mount(
        "/",
        routes![
            controller::student_controller::index,
        ],
    )
    .attach(controller::student_controller::stage())
}

