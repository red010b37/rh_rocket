use crate::errors::our_error::OurError;
use crate::states::Directus;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Utc};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use regex::Regex;
use rocket::form::{self, Error as FormError, FromForm};
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::models::jobs_countries::JobCountry;
use crate::models::jobs_regions::JobRegion;
use crate::models::jobs_tags::JobTags;
use crate::models::query_resp::{Daum, GetJobsResult};
use crate::models::tag::Tag;
use crate::uilts::is_valid_guid;
use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;
use rocket::tokio::task;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManyJobsResult {
    pub data: Vec<Job>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleJobResult {
    pub data: Job,
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
pub struct NewJobForm<'r> {
    #[field(validate = len(3..100).or_else(msg!("company name cannot be empty")))]
    pub company_name: &'r str,
    pub position: &'r str,
    pub position_type: &'r str,
    pub category: &'r str,
    pub min_per_year: i32,
    pub max_per_year: i32,
    pub job_description: String,
    pub how_to_apply: Option<String>,
    pub apply_url: Option<String>,
    pub apply_email: Option<String>,
    pub location: Vec<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJobPost {
    pub company_name: String,
    pub position: String,
    pub position_type: String,
    pub category: String,
    // pub location: Vec<String>,
    pub min_per_year: i32,
    pub max_per_year: i32,
    pub description: String,
    pub how_to_apply: Option<String>,
    pub apply_url: Option<String>,
    pub apply_email: Option<String>,
    pub gen_id: i32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJobResult {
    pub data: Job,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub id: String,
    pub status: String,
    pub company_name: String,
    pub position: String,
    pub position_type: String,
    pub category: String,
    // pub location: Vec<String>,
    pub min_per_year: i32,
    pub max_per_year: i32,
    pub description: String,
    pub how_to_apply: Option<String>,
    pub apply_url: Option<String>,
    pub apply_email: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub date_created: DateTime<Utc>,
    pub date_updated: Option<DateTime<Utc>>,
}

impl Job {
    pub async fn create<'r>(
        new_job: &'r NewJobForm<'r>,
        directus: &State<Directus>,
    ) -> Result<Self, OurError> {
        let mut tags: Vec<String> = Vec::new();
        let mut tags_to_create: Vec<String> = Vec::new();

        // look to see if we have any new tags to create
        if !new_job.tags.is_none() {
            for tag_id in new_job.tags.as_ref().unwrap().iter() {
                // println!("{:?}", tag_id);
                // tags.push(tag_id.to_string());
                if is_valid_guid(tag_id) {
                    println!("{:?}", true);
                    tags.push(tag_id.to_string())
                } else {
                    println!("{:?} needs to be create", tag_id);
                    tags_to_create.push(tag_id.to_string());
                }
            }
        }

        // if we have fund some tags to create post them
        if !tags_to_create.is_empty() {
            let create_tags = Tag::create_tags(directus, tags_to_create).await?;
            // println!("Created tagd {:?}", create_tags);
            for new_tag in create_tags {
                tags.push(new_tag.id)
            }
        }

        let mut regions: Vec<i32> = Vec::new();
        let mut countries: Vec<i32> = Vec::new();

        // sort out the regions and countires
        for location in new_job.location.iter() {
            if location.starts_with('r') {
                println!("{:?}", location);
                let locationId: i32 = location.split('_').nth(1).unwrap().parse().unwrap();
                println!("{:?}", locationId);
                regions.push(locationId);
            }

            if location.starts_with('c') {
                println!("{:?}", location);
                let countryId: i32 = location.split('_').nth(1).unwrap().parse().unwrap();
                println!("{:?}", countryId);
                countries.push(countryId);
            }
        }

        let random_number = task::spawn_blocking(|| {
            let mut rng = thread_rng();
            rng.gen_range(10000..100000)
        })
        .await
        .unwrap();

        let num_i32: i32 = random_number as i32;
        println!("Random 5-digit number: {}", num_i32);

        let slug = Self::gen_slug(new_job.position.parse().unwrap(), num_i32);

        let dPost = CreateJobPost {
            company_name: new_job.company_name.parse().unwrap(),
            position: new_job.position.parse().unwrap(),
            position_type: new_job.position_type.parse().unwrap(),
            category: new_job.category.parse().unwrap(),
            min_per_year: new_job.max_per_year,
            max_per_year: new_job.max_per_year,
            description: new_job.job_description.parse().unwrap(),
            how_to_apply: new_job.how_to_apply.clone(),
            apply_url: new_job.apply_url.clone(),
            apply_email: new_job.apply_email.clone(),
            gen_id: num_i32,
            slug,
        };

        // create the job
        let post_url = directus.directus_api_url.to_string() + "/items/jobs";
        let create_job_result: CreateJobResult = reqwest::Client::new()
            .post(post_url)
            .json(&dPost)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        let job_id = create_job_result.data.id.to_string();

        // connect the job and the tags together
        if !tags.is_empty() {
            JobTags::create(directus, job_id.to_string(), tags).await?;
        }

        // connect the regions
        if !regions.is_empty() {
            JobRegion::create(directus, job_id.to_string(), regions).await?
        }

        // connect the countries
        if !countries.is_empty() {
            JobCountry::create(directus, job_id.to_string(), countries).await?
        }

        println!("{:?}", create_job_result);

        Ok(create_job_result.data)
    }

    pub async fn get_jobs<'r>(directus: &State<Directus>) -> Result<Vec<Daum>, OurError> {
        let mut url = directus.directus_api_url.to_string() + "/items/jobs";
        url += "?fields=id,status,position,company_name,min_per_year,max_per_year,slug,date_updated,date_created,tags.tag_id.name,tags.tag_id.id,region.region_id.id,region.region_id.name,countries.country_id.id,countries.country_id.name";

        println!("{:?}", url);
        let result: GetJobsResult = reqwest::Client::new()
            .get(url)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        Ok(result.data)
    }

    fn gen_slug(title: String, gen_id: i32) -> String {
        // Make the string lowercase
        let s_lower = title.to_lowercase();

        // Remove all characters except for letters, numbers, and spaces
        let s_filtered = Regex::new(r"[^a-z0-9\s]")
            .unwrap()
            .replace_all(&s_lower, "");

        // Replace multiple spaces with a single hyphen
        let s_hyphenated = Regex::new(r"\s+").unwrap().replace_all(&s_filtered, "-");

        // Convert to a String
        let mut s_final: String = s_hyphenated.into();

        s_final = format!("{}-{}", s_final, gen_id.to_string());
        println!("Modified string: {}", s_final);
        return s_final;
    }
}
