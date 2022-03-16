mod controller;
mod model;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            controller::greeting_controller::index,
            controller::greeting_controller::delay,
        ],
    )
    .attach(controller::greeting_controller::stage())
}

