use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use crate::errors::our_error::OurError;
use crate::states::AppSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManyTagsResult {
    pub data: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleTagsResult {
    pub data: Tag,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub status: String,
    pub sort: Option<i32>,
    pub date_created: DateTime<Utc>,
    pub date_updated: Option<DateTime<Utc>>,
}

pub async fn get_all<'r>(
    app_settings: &State<AppSettings>,
) -> Result<Vec<Tag>, OurError> {
    let url = url_path(app_settings);
    println!("{:?}", url);
    let result: ManyTagsResult = reqwest::Client::new()
        .get(url)
        .bearer_auth(app_settings.directus.token.to_string())
        .send()
        .await?
        .json()
        .await?;

    Ok(result.data)
}

fn url_path (app_settings: &State<AppSettings>) -> String {
    app_settings.directus.directus_api_url.to_string() + "/items/tags?limit=-1"
}
