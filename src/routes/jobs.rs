use super::HtmlResponse;
use crate::models::{
    job::{Job, NewJobForm},
    tag::Tag,
};
use crate::states::Directus;
use rocket::http::Status;
use rocket::{
    form::{Contextual, Form},
    State,
};
use rocket::tokio::{join, try_join};
use rocket_dyn_templates::{context, Template};
use crate::errors::our_error::OurError;
use crate::models::countries::Country;
use crate::models::region::Region;

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

    // get the tags
    let tags = Tag::get_all(directus)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let regions = Region::get_all(directus)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let countries = Country::get_all(directus)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render(
        "jobs/hire",
        context! {
            tags: tags,
            regions: regions,
            countries: countries,
        },
    ))
}

#[post(
"/hire-remotely",
format = "application/x-www-form-urlencoded",
data = "<job_context>"
)]
pub async fn create_new_job<'r>(
    job_context: Form<Contextual<'r, NewJobForm<'r>>>,
    directus: &State<Directus>, // csrf_token: CsrfToken,
) -> &'static str {
    let new_job = job_context.value.as_ref().unwrap();

    Job::create(new_job, directus).await.map_err(|_| {
        print!("{:?}", "errrorsss");
    });

    "post"
}
