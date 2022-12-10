use crate::errors::our_error::OurError;
use crate::states::Directus;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Utc};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JobTags {
    pub id: String,
    pub job_id: String,
    pub tag_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJobTags {
    pub job_id: String,
    pub tag_id: String,
}

impl JobTags {
    pub async fn create(
        directus: &State<Directus>,
        job_id: String,
        tag_ids: Vec<String>,
    ) -> Result<(), OurError> {
        let mut data_to_send: Vec<CreateJobTags> = Vec::new();

        for tag_id in tag_ids {
            data_to_send.push(CreateJobTags {
                tag_id,
                job_id: job_id.to_string(),
            });
        }

        let post_url = directus.directus_api_url.to_string() + "/items/jobs_tags";
        let respose = reqwest::Client::new()
            .post(post_url)
            .json(&data_to_send)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?;

        Ok(())
    }
}
