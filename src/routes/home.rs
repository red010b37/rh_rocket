use super::HtmlResponse;
use crate::states::{AppSettings, Directus};
use rocket::http::Status;
use rocket::State;
use rocket_dyn_templates::{context, Template};

// #[get("/", format = "text/html")]
// pub async fn homepage(
//     directus: &State<Directus>,
//     app_settings: &State<AppSettings>,
// ) -> HtmlResponse {
//     let jobs = Job::get_jobs(directus, app_settings)
//         .await
//         .map_err(|_| Status::InternalServerError)?;
//
//     let context = context! {
//         jobs,
//     };
//
//     Ok(Template::render("index", &context))
// }

#[get("/do_health_check")]
pub async fn health_check() -> &'static str {
    ""
}
