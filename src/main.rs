#[macro_use]
extern crate rocket;
extern crate rand;
extern crate regex;

use remote_hut::setup_rocket;
use rocket::{Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    setup_rocket().await
}
