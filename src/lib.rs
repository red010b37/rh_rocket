#[macro_use]
extern crate rocket;

use crate::routes::{home, jobs, metrics};
use crate::states::{AppSettings, Clockify, Directus};
use log::LevelFilter;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};
use rocket_dyn_templates::tera::Function;
use rocket_dyn_templates::Template;
use std::collections::BTreeMap;

pub mod commands;
pub mod directus_res;
pub mod errors;
pub mod models;
pub mod routes;
pub mod services;
pub mod states;
pub mod uilts;

#[derive(Deserialize)]
pub struct Config {
    directus_token: String,
    directus_api_url: String,
    clockify_token: String,
    rh_env: String,
}

fn setup_logger() {
    let (level, logger) = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}] [{level}][{target}] [{message}]",
                date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            fern::log_file("logs/application.log")
                .unwrap_or_else(|_| panic!("Cannot open logs/application.log")),
        )
        .into_log();
    async_log::Logger::wrap(logger, || 0).start(level).unwrap();
}

pub async fn setup_rocket() -> Rocket<Build> {
    // setup_logger();
    let our_rocket = rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                // home::homepage,
                home::health_check,
            ],
        )
        .mount(
            "/",
            routes![
                jobs::index,
                jobs::new_job,
                jobs::create_new_job,
                // jobs::view_job,
            ],
        )
        .mount("/", routes![metrics::index, metrics::clockfyCron,])
        .mount("/", FileServer::from(relative!("static")));

    // Load the config
    let config: Config = our_rocket
        .figment()
        .extract()
        .expect("Incorrect Rocket.toml configuration");

    let directus = Directus {
        token: config.directus_token.clone(),
        directus_api_url: config.directus_api_url.clone(),
    };

    let clockify = Clockify {
        token: config.clockify_token.clone(),
    };

    let app_settings = AppSettings {
        env: config.rh_env.clone(),
        clockify: clockify.clone(),
        directus: directus.clone(),
    };

    // pass our dtat for rocket to manage in state for us
    let final_rocket = our_rocket
        .manage(directus)
        .manage(clockify)
        .manage(app_settings);

    final_rocket
}
