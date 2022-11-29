#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use log::LevelFilter;


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


#[launch]
async fn rocket() -> Rocket<Build> {
    setup_logger();
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, RemoteHub!"
}