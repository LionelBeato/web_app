mod controller;
mod model;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            controller::student_controller::index,
            controller::student_controller::delay,
        ],
    )
    .attach(controller::student_controller::stage())
}

