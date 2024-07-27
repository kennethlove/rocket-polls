#[macro_use] extern crate rocket;

use polls::polls::index as polls_index;
use polls::polls::detail as polls_detail;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        polls_index,
        polls_detail
    ])
}
