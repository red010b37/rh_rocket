use crate::errors::our_error::OurError;
use crate::states::AppSettings;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJobTags {
    pub job_id: String,
    pub tag_id: String,
}


/*
pub async fn create(
    app_settings: &State<AppSettings>,
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

    let post_url = app_settings.directus.directus_api_url.to_string() + "/items/jobs_tags";
    let respose = reqwest::Client::new()
        .post(post_url)
        .json(&data_to_send)
        .bearer_auth(app_settings.directus.token.to_string())
        .send()
        .await?;

    Ok(())
}
 */
