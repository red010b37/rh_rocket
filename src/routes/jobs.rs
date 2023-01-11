use rocket::{
    form::{Contextual, Form},
    State,
};
use rocket::http::Status;
use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};
use serde::__private::ser::serialize_tagged_newtype;
use serde::Deserialize;
use thousands::Separable;

use crate::commands::{command_create_job, command_prep_hire_page};
use crate::errors::our_error::OurError;
use crate::services::tags_svc;
use crate::states::{AppSettings, Directus};

use super::HtmlResponse;

#[get("/job")]
pub async fn index(directus: &State<Directus>) -> &'static str {
    //    println!("{}", &directus.token.to_string());
    //    Job::create(
    //            &NewJob { company_name: "d" },
    //    directus.token.to_string(),
    //    ).await;
    "Im the job"
}

/*
#[get("/remote-job/<slug>")]
pub async fn view_job(
    directus: &State<Directus>,
    app_settings: &State<AppSettings>,
    slug: &str,
) -> HtmlResponse {
    println!("{:?}", slug);

    let job_item = Job::get_job(directus, app_settings, slug.to_string()).await?;

    let formatted_min = job_item.min_per_year.clone().separate_with_commas();
    let formatted_max = job_item.max_per_year.clone().separate_with_commas();

    let clean_description = ammonia::clean(&*job_item.description.clone());
    let clean_apply = ammonia::clean(&*job_item.how_to_apply.clone());

    Ok(Template::render(
        "jobs/view",
        context! {
            job_item,
            formatted_min_per_year: formatted_min,
            formatted_max_per_year: formatted_max,
            clean_description,
            clean_apply,
        },
    ))
}
*/

#[get("/hire-remotely")]
pub async fn new_job(app_settings: &State<AppSettings>) -> HtmlResponse {
    let data = command_prep_hire_page::execute(app_settings)
        .await
        .map_err(|_| Status::InternalServerError)?;

    // get the tags
    // let tags = tags_svc::get_all(app_settings)
    //     .await
    //     .map_err(|_| Status::InternalServerError)?;

    // let regions = Region::get_all(app_settings.directus)
    //     .await
    //     .map_err(|_| Status::InternalServerError)?;
    // //
    // let countries = Country::get_all(app_settings.directus)
    //     .await
    //     .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render(
        "jobs/hire",
        context! {
            tags: data.tags,
            regions: data.regions,
            countries: data.countries,
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
    directus: &State<Directus>,        // csrf_token: CsrfToken,
    app_settings: &State<AppSettings>, // csrf_token: CsrfToken,
) -> &'static str {
    let new_job = job_context.value.as_ref().unwrap();

    command_create_job::execute(app_settings, new_job)
        .await
        .map_err(|_| {
            print!("{:?}", "errrorsss");
        });

    /*

    Job::create(new_job, directus, app_settings)
        .await
        .map_err(|_| {
            print!("{:?}", "errrorsss");
        });


     */

    "post"
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
pub struct NewJobForm<'r> {
    #[field(validate = len(3..100).or_else(msg ! ("company name cannot be empty")))]
    pub company_name: &'r str,
    pub company_url: Option<String>,
    pub position: &'r str,
    pub position_type: &'r str,
    pub category: &'r str,
    pub min_per_year: i32,
    pub max_per_year: i32,
    pub job_description: String,
    pub how_to_apply: Option<String>,
    pub apply_url: Option<String>,
    pub apply_email: Option<String>,
    pub location: Vec<String>,
    pub tags: Option<Vec<String>>,
}
