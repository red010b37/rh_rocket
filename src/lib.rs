#[macro_use]
extern crate rocket;

use crate::states::Directus;
use crate::routes::{jobs};
use log::LevelFilter;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::serde::Deserialize;
use rocket::State;
use rocket::{Build, Rocket};




pub mod errors;
pub mod routes;
pub mod models;
pub mod states;



#[derive(Deserialize)]
pub struct Config {
    directus_token: String,
}


fn setup_logger() {
    let (level, logger) = fern::Dispatch::new()
    .format(move | out, message, record| {
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
    setup_logger();
    let our_rocket = rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![jobs::index]);

    // Load the config
    let config: Config = our_rocket
    .figment()
    .extract()
    .expect("Incorrect Rocket.toml configuration");

    let directus = Directus {
        token: config.directus_token.clone(),
    };

    // pass our dtat for rocket to manage in state for us
    let final_rocket = our_rocket.manage(directus);
    final_rocket
}

#[get("/")]
fn index(directus: &State<Directus>) -> &'static str {
    println!("{}", &directus.token.to_string());
    "Hello, RemoteHub!"
}