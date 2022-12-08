
use super::HtmlResponse;
use crate::models::{
    job::{Job, NewJobForm},
};
use crate::states::Directus;
use rocket::{State, form::{Contextual, Form}};
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


#[post(
    "/hire-remotely",
    format = "application/x-www-form-urlencoded",
    data = "<job_context>"
)]
pub async fn create_new_job<'r>(
    job_context: Form<Contextual<'r, NewJobForm<'r>>>,
    directus: &State<Directus>
    // csrf_token: CsrfToken,
) -> &'static str {

    let new_job = job_context.value.as_ref().unwrap();

    Job::create(new_job, directus).await.map_err(|_| {
        print!("{:?}", "errrorsss");
    });

    "post"
}

