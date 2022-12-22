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

impl Region {
    pub async fn get_all<'r>(
        directus: &State<Directus>,
    ) -> Result<Vec<Self>, OurError> {
        let url = Self::url_path(directus);
        println!("{:?}", url);
        let result: ManyRegionResult = reqwest::Client::new()
            .get(url)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        // println!("{:?}", result);
        Ok(result.data)
    }


    fn url_path (directus: &State<Directus>) -> String {
        directus.directus_api_url.to_string() + "/items/region"
    }
}
