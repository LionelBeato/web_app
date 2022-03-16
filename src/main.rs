mod controller;
mod model;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    // let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    // let config = aws_config::from_env().region(region_provider).load().await;
    // let client = Client::new(&config); // passing config by reference
    rocket::build().mount(
        "/",
        routes![
            controller::student_controller::index,
            controller::student_controller::delay,
        ],
    )
    .attach(controller::student_controller::stage())
}

