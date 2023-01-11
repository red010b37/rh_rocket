use rocket::form::{self, Error as FormError, FromForm};
use serde::{Serialize, Deserialize};
use chrono::{Datelike, DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use crate::errors::our_error::OurError;
use crate::states::{AppSettings, Directus};
use rocket::State;

use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct ManyCountriesResult {
    pub data: Vec<Country>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleCountryResult {
    pub data: Country,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub code: String,
    pub symbol: String,
    pub date_created: DateTime<Utc>,
    pub date_updated: Option<DateTime<Utc>>,
}

pub async fn get_all<'r>(
    app_settings: &State<AppSettings>,
) -> Result<Vec<Country>, OurError> {
    let url = url_path(app_settings);
    println!("{:?}", url);
    let result: ManyCountriesResult = reqwest::Client::new()
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
    app_settings.directus.directus_api_url.to_string() + "/items/countries?limit=-1"
}

