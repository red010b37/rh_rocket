use crate::errors::our_error::OurError;
use crate::states::AppSettings;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJobPost {
    pub status: String,
    pub company_name: String,
    pub company_url: Option<String>,
    pub position: String,
    pub position_type: String,
    pub category: String,
    // pub location: Vec<String>,
    pub min_per_year: i32,
    pub max_per_year: i32,
    pub description: String,
    pub how_to_apply: Option<String>,
    pub apply_url: Option<String>,
    pub apply_email: Option<String>,
    pub gen_id: i32,
    pub slug: String,
    pub expires_date: Option<DateTime<Utc>>,
    pub publish_date: Option<DateTime<Utc>>,
    pub env: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJobResult {
    pub data: CreateJobR,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJobR {
    pub id: String,
}

pub async fn create_job(
    post: CreateJobPost,
    app_settings: &State<AppSettings>,
) -> Result<String, OurError> {
    let post_url = app_settings.directus.directus_api_url.to_string() + "/items/jobs";

    let create_job_result: CreateJobResult = reqwest::Client::new()
        .post(post_url)
        .json(&post)
        .bearer_auth(app_settings.directus.token.to_string())
        .send()
        .await?
        .json()
        .await?;

    let job_id = create_job_result.data.id.to_string();

    Ok(job_id)
}

fn url_path(app_settings: &State<AppSettings>) -> String {
    app_settings.directus.directus_api_url.to_string() + "/items/jobs"
}

fn build_query_url(
    app_setting: &State<AppSettings>,
    limit: i32,
    filter: Option<String>,
    sort: Option<String>,
    app_settings: &State<AppSettings>,
) -> String {
    let fields = "?fields=*,tags.tag_id.name,tags.tag_id.id,region.region_id.id,region.region_id.name,countries.country_id.id,countries.country_id.name";

    let mut url = url_path(app_setting) + fields + "&limit=" + &*limit.to_string();

    if !filter.is_none() {
        url = url + "&" + &*filter.unwrap().to_string()
    }

    // add the env filter so test or prod jobs are retured
    url = url + "&filter[env][_eq]=" + &*app_settings.env;

    if !sort.is_none() {
        url = url + "&" + &*sort.unwrap().to_string()
    }

    println!("{:?}", url);

    url
}
