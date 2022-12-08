
use rocket::form::{self, Error as FormError, FromForm};
use serde::{Serialize, Deserialize};
use chrono::{Datelike, DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use crate::errors::our_error::OurError;
use crate::states::{Directus};
use rocket::State;

use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;

#[derive(Debug, FromForm, Serialize,  Deserialize)]
pub struct NewJobForm<'r> {
    #[field(validate = len(3..100).or_else(msg!("company name cannot be empty")))]
    pub company_name: &'r str,
    pub position: &'r str,
    pub position_type: &'r str,
    pub category: &'r str,
    pub min_per_year: i32,
    pub max_per_year: i32,
    pub description: String,
    pub how_to_apply: Option<String>,
    pub apply_url: Option<String>,
    pub apply_email: Option<String>,
    pub location: Vec<String>,
}

#[derive(Debug, Serialize,  Deserialize)]
pub struct CreateJobResult {
    pub data: Job,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Job{
    pub company_name: String,
    pub position: String,
    pub position_type: String,
    pub category: String,
    pub location: String,
    pub min_per_year: i32,
    pub max_per_year: i32,
    pub description: String,
    pub how_to_apply: Option<String>,
    pub apply_url: Option<String>,
    pub apply_email: Option<String>,
    pub date_created: DateTime<Utc>,
    pub date_updated: Option<DateTime<Utc>>,
}

impl Job {

    pub async fn create<'r>(
            new_job: &'r NewJobForm<'r>,
            directus: &State<Directus>,
    ) -> Result<Self, OurError> {

        let post_url = directus.directus_api_url.to_string() + "/items/jobs";
        let create_job_result: CreateJobResult = reqwest::Client::new()
            .post(post_url)
            .json(&new_job)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;
            

            println!("{:?}", create_job_result);

            Ok(create_job_result.data)
        
    }

}
