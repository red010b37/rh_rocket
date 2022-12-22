use crate::errors::our_error::OurError;
use crate::states::Directus;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Utc};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JobCountry {
    pub id: String,
    pub job_id: String,
    pub country_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJobCountry {
    pub job_id: String,
    pub country_id: i32,
}

impl JobCountry {
    pub async fn create(
        directus: &State<Directus>,
        job_id: String,
        countries_ids: Vec<i32>,
    ) -> Result<(), OurError> {
        let mut data_to_send: Vec<CreateJobCountry> = Vec::new();

        for region_id in countries_ids {
            data_to_send.push(CreateJobCountry {
                country_id: region_id,
                job_id: job_id.to_string(),
            });
        }

        let post_url = directus.directus_api_url.to_string() + "/items/jobs_countries";
        let respose = reqwest::Client::new()
            .post(post_url)
            .json(&data_to_send)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?;

        Ok(())
    }
}
