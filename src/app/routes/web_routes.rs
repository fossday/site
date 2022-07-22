use crate::app::controllers::page_controller::get_home;
use rocket::get;
use rocket_dyn_templates::Template;

#[get("/")]
pub async fn home() -> Template {
    return get_home().await;
}
