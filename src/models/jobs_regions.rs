use crate::errors::our_error::OurError;
use crate::states::Directus;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Utc};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JobRegion {
    pub id: String,
    pub job_id: String,
    pub region_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJobRegions {
    pub job_id: String,
    pub region_id: i32,
}

impl JobRegion {
    pub async fn create(
        directus: &State<Directus>,
        job_id: String,
        region_ids: Vec<i32>,
    ) -> Result<(), OurError> {
        let mut data_to_send: Vec<CreateJobRegions> = Vec::new();

        for region_id in region_ids {
            data_to_send.push(CreateJobRegions {
                region_id,
                job_id: job_id.to_string(),
            });
        }

        let post_url = directus.directus_api_url.to_string() + "/items/jobs_regions";
        let respose = reqwest::Client::new()
            .post(post_url)
            .json(&data_to_send)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?;

        Ok(())
    }
}
