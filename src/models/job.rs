/*
use crate::errors::our_error::OurError;
use crate::states::{AppSettings, Directus};
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Utc};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use regex::Regex;
use rocket::form::{self, Error as FormError, FromForm};
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::directus_res::jobs_view_result::{JobQueryResults, JobResultItem};
use crate::models::jobs_countries::JobCountry;
use crate::models::jobs_regions::JobRegion;
use crate::models::jobs_tags::JobTags;
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
    #[field(validate = len(3..100).or_else(msg ! ("company name cannot be empty")))]
    pub company_name: &'r str,
    pub company_url: Option<String>,
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
    pub status: String,
    pub company_name: String,
    pub company_url: Option<String>,
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
    pub expires_date: Option<DateTime<Utc>>,
    pub publish_date: Option<DateTime<Utc>>,
    pub env: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJobResult {
    pub data: CreateJobR,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJobR {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {}

impl Job {
    pub async fn create<'r>(
        new_job: &'r NewJobForm<'r>,
        directus: &State<Directus>,
        app_settings: &State<AppSettings>,
    ) -> Result<String, OurError> {
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

        let clean_desc = ammonia::clean(&*new_job.job_description.to_string());

        let mut cloned_opt = new_job.how_to_apply.clone();
        let apply_data = cloned_opt.get_or_insert("".to_string());
        let clean_apply = &mut ammonia::clean(apply_data);

        let publish_date = Utc::now();
        let expires_date = Utc::now() + Duration::days(30);

        let dPost = CreateJobPost {
            status: "published".to_string(),
            company_name: new_job.company_name.parse().unwrap(),
            company_url: new_job.company_url.clone(),
            position: new_job.position.parse().unwrap(),
            position_type: new_job.position_type.parse().unwrap(),
            category: new_job.category.parse().unwrap(),
            min_per_year: new_job.min_per_year,
            max_per_year: new_job.max_per_year,
            description: clean_desc,
            how_to_apply: Some((*clean_apply.clone()).parse().unwrap()),
            apply_url: new_job.apply_url.clone(),
            apply_email: new_job.apply_email.clone(),
            gen_id: num_i32,
            slug,
            publish_date: Some(publish_date),
            expires_date: Some(expires_date),
            env: app_settings.env.clone(),
        };

        // create the job
        let post_url = directus.directus_api_url.to_string() + "/items/jobs";

        println!("posting");
        // let s: String = reqwest::Client::new()
        //     .post(post_url.clone())
        //     .json(&dPost)
        //     .bearer_auth(directus.token.to_string())
        //     .send()
        //     .await?
        //     .text()
        //     .await?;
        //
        // println!("{:?}", s);

        let create_job_result: CreateJobResult = reqwest::Client::new()
            .post(post_url)
            .json(&dPost)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        let job_id = create_job_result.data.id.to_string();

        println!("C1");

        // connect the job and the tags together
        if !tags.is_empty() {
            println!("C2");
            JobTags::create(directus, job_id.to_string(), tags).await?;
            println!("C3");
        }

        // connect the regions
        if !regions.is_empty() {
            println!("C4");
            JobRegion::create(directus, job_id.to_string(), regions).await?;
            println!("C5");
        }

        // connect the countries
        if !countries.is_empty() {
            println!("C6");
            JobCountry::create(directus, job_id.to_string(), countries).await?;
            println!("C7");
        }

        println!("{:?}", create_job_result);

        Ok(job_id)
    }

    pub async fn get_jobs<'r>(
        directus: &State<Directus>,
        app_settings: &State<AppSettings>,
    ) -> Result<Vec<JobResultItem>, OurError> {
        let mut url = Self::build_query_url(
            directus,
            -1,
            Option::from("filter[status][_eq]=published".to_string()),
            Option::from("sort=-publish_date".to_string()),
            app_settings,
        );

        let result: JobQueryResults = reqwest::Client::new()
            .get(url)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        Ok(result.data)
    }

    pub async fn get_job<'r>(
        directus: &State<Directus>,
        app_settings: &State<AppSettings>,
        slug: String,
    ) -> Result<JobResultItem, OurError> {
        let url = Self::build_query_url(
            directus,
            1,
            Option::from(format!("filter[slug][_eq]={}", slug)),
            None,
            app_settings,
        );

        let result: JobQueryResults = reqwest::Client::new()
            .get(url)
            .bearer_auth(directus.token.to_string())
            .send()
            .await?
            .json()
            .await?;

        let d = match result.data.get(0) {
            Some(v) => v,
            None => {
                return Err(OurError::new_not_found_error(
                    String::from("Job not found"),
                    None,
                ))
            }
        };
        let r = d.clone();
        Ok(r)
    }

    fn url_path(directus: &State<Directus>) -> String {
        directus.directus_api_url.to_string() + "/items/jobs"
    }

    fn build_query_url(
        directus: &State<Directus>,
        limit: i32,
        filter: Option<String>,
        sort: Option<String>,
        app_settings: &State<AppSettings>,
    ) -> String {
        let fields = "?fields=*,tags.tag_id.name,tags.tag_id.id,region.region_id.id,region.region_id.name,countries.country_id.id,countries.country_id.name";

        let mut url = Self::url_path(directus) + fields + "&limit=" + &*limit.to_string();

        if !filter.is_none() {
            url = url + "&" + &*filter.unwrap().to_string()
        }

        // add the env filter so test or prod jobs are retured
        url = url + "&filter[env][_eq]=" + &*app_settings.env;

        if !sort.is_none() {
            url = url + "&" + &*sort.unwrap().to_string()
        }

        println!("{:?}", url);

        url
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


 */
