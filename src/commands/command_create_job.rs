use crate::errors::our_error::OurError;
use crate::routes::jobs::NewJobForm;
use crate::services::job_svc;
use crate::states::AppSettings;
use crate::uilts::{gen_slug, is_valid_guid};
use chrono::{DateTime, Duration, Utc};
use rand::{thread_rng, Rng};
use rocket::tokio::task;
use rocket::State;

pub async fn execute<'r>(
    app_settings: &State<AppSettings>,
    new_job: &'r NewJobForm<'r>,
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
    // if !tags_to_create.is_empty() {
    //     let create_tags = Tag::create_tags(directus, tags_to_create).await?;
    //     // println!("Created tagd {:?}", create_tags);
    //     for new_tag in create_tags {
    //         tags.push(new_tag.id)
    //     }
    // }

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

    let slug = gen_slug(new_job.position.parse().unwrap(), num_i32);

    let clean_desc = ammonia::clean(&*new_job.job_description.to_string());

    let mut cloned_opt = new_job.how_to_apply.clone();
    let apply_data = cloned_opt.get_or_insert("".to_string());
    let clean_apply = &mut ammonia::clean(apply_data);

    let publish_date = Utc::now();
    let expires_date = Utc::now() + Duration::days(30);

    let dPost = job_svc::CreateJobPost {
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

    let job_id = job_svc::create_job(dPost, app_settings).await?;

    Ok(job_id)
}
