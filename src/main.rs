#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

pub mod app;
use app::utils::hbs;

use app::routes::web_routes::home;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", FileServer::from("static"))
    .mount("/", routes![home])
    .attach(Template::custom(|engines| {
        hbs::customize(&mut engines.handlebars);
    }))
}