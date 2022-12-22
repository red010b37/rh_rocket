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
struct NewTag {
    name: String,
}

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

impl Tag {
    pub async fn get_all<'r>(
        directus: &State<Directus>,
    ) -> Result<Vec<Self>, OurError> {
        let url = Self::url_path(directus);
        println!("{:?}", url);
        let result: ManyTagsResult = reqwest::Client::new()
            .get(url)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        // println!("{:?}", result);
        Ok(result.data)
    }

    pub async fn create_tags(
        directus: &State<Directus>,
        names: Vec<String>
    ) -> Result<Vec<Self>, OurError> {
        let client = reqwest::Client::new();

        let tags: Vec<NewTag> = names.into_iter().map(|name| NewTag { name }).collect();
        let url = Self::url_path(directus);

        let response: ManyTagsResult = client
            .post(url)
            .bearer_auth(directus.token.to_string())
            .json(&tags)
            .send()
            .await?
            .json()
            .await?;

        //
        // let mut return_data: Vec<Self> = Vec::new();
        // match response {
        //     Ok(many_tags) => {
        //         println!("{:?}", many_tags);
        //         return_data = many_tags.data;
        //     },
        //     Err(sinle_tag) => {
        //         println!("{:?}", sinle_tag);
        //         return_data.push(sinle_tag.data);
        //     }
        // }

        Ok(response.data)
    }

    fn url_path (directus: &State<Directus>) -> String {
        directus.directus_api_url.to_string() + "/items/tags?limit=-1"
    }
}
