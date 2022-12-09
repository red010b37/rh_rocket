use rocket::form::{self, Error as FormError, FromForm};
use serde::{Serialize, Deserialize};
use chrono::{Datelike, DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use crate::errors::our_error::OurError;
use crate::states::{Directus};
use rocket::State;

use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct GetTagsResult {
    pub data: Vec<Tag>,
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

impl Tag {
    pub async fn get_all<'r>(
        directus: &State<Directus>,
    ) -> Result<Vec<Self>, OurError> {
        let url = directus.directus_api_url.to_string() + "/items/tags";
        println!("{:?}", url);
        let result: GetTagsResult = reqwest::Client::new()
            .get(url)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        // println!("{:?}", result);
        Ok(result.data)
    }
}
