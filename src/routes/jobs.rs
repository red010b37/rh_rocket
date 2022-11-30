
use crate::models::{
    job::{Job, NewJob},
};
use crate::states::Directus;
use rocket::State;


#[get("/job")]
pub async fn index(directus: &State<Directus>) -> &'static str {
//    println!("{}", &directus.token.to_string());
    Job::create(
            &NewJob { company_name: "d" }, directus.token.to_string()
    ).await?;
    "Im the job"
}