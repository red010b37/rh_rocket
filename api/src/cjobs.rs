use rocket::fairing::{self, AdHoc};
use rocket::form::{Context, Form};
use rocket::fs::{relative, FileServer};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::Template;
use rocket_example_core::{Mutation, Query};
use serde_json::json;

use migration::MigratorTrait;
use sea_orm_rocket::{Connection, Database};

pub use entity::job;
pub use entity::job::Entity as Job;

use crate::pool;
use pool::Db;

#[get("/hire-remotely")]
pub async fn new() -> Template {
    Template::render("jobs/hire", &Context::default())
}

#[post("/hire-remotely", data = "<create_job>")]
pub async fn create(conn: Connection<'_, Db>, create_job: Form<CreateJob>) -> Template {

    // let j = create_job.company_name;

    // println!("{j}")
    Template::render("jobs/hire", &Context::default())
}



#[derive(FromForm, Clone)]
pub struct CreateJob {
    company_name:  String,
}