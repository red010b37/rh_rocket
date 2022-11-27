#[macro_use]
extern crate rocket;

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

mod pool;
use pool::Db;

pub use entity::post;
pub use entity::post::Entity as Post;

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

mod posts;

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        json! ({
            "uri": req.uri()
        }),
    )
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", FileServer::from(relative!("/static")))
        .mount(
            "/posts",
            routes![
                posts::new,
                posts::create,
                posts::delete,
                posts::destroy,
                posts::list,
                posts::edit,
                posts::update
            ],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await
        .map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
