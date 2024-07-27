#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;

use polls::polls::list as polls_list;
use polls::polls::detail as polls_detail;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        polls_list,
        polls_detail
    ])
    .attach(Template::fairing())
}
