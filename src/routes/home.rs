use rocket::http::Status;
use rocket::State;
use super::HtmlResponse;
use rocket_dyn_templates::{context, Template};
use crate::models::job::Job;
use crate::states::Directus;

#[get("/", format = "text/html")]
pub async fn homepage(directus: &State<Directus>) -> HtmlResponse {

    let jobs = Job::get_jobs(directus)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let context = context! {
        jobs,
    };

    Ok(Template::render("index", &context))
}


#[get("/do_health_check")]
pub async fn health_check() -> &'static str {
    ""
}
