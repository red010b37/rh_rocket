use rocket::form::{Error as FormError, FromForm};
use serde::{Serialize, Deserialize};
use chrono::{Datelike, DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use crate::errors::our_error::OurError;
use crate::states::{AppSettings, Directus};
use rocket::State;

use reqwest;


#[derive(Debug, Serialize, Deserialize)]
pub struct ManyRegionResult {
    pub data: Vec<Region>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleRegionResult {
    pub data: Region,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub code: String,
    pub date_created: DateTime<Utc>,
    pub date_updated: Option<DateTime<Utc>>,
}


pub async fn get_all<'r>(
    app_settings: &State<AppSettings>,
) -> Result<Vec<Region>, OurError> {
    let url = url_path(app_settings);
    println!("{:?}", url);
    let result: ManyRegionResult = reqwest::Client::new()
        .get(url)
        .bearer_auth(app_settings.directus.token.to_string())
        .send()
        .await?
        .json()
        .await?;

    // println!("{:?}", result);
    Ok(result.data)
}


fn url_path(app_settings: &State<AppSettings>) -> String {
    app_settings.directus.directus_api_url.to_string() + "/items/region"
}

