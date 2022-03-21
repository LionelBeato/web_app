mod controller;
mod model;


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
        ],
    )
    .attach(controller::student_controller::stage())
}

