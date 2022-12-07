
use super::HtmlResponse;
use crate::models::{
    job::{Job, NewJob},
};
use crate::states::Directus;
use rocket::State;
use rocket_dyn_templates::{context, Template};


#[get("/job")]
pub async fn index(directus: &State<Directus>) -> &'static str {
//    println!("{}", &directus.token.to_string());
//    Job::create(
//            &NewJob { company_name: "d" },
//    directus.token.to_string(),
//    ).await;
    "Im the job"
}

#[get("/hire-remotely")]
pub async fn new_job(directus: &State<Directus>) -> HtmlResponse {
    Ok(Template::render("jobs/hire", context! {}))
}
