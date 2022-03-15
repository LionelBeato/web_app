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
            controller::greeting_controller::get_student
        ],
    )
}
