
use rocket::State;
use crate::states:: {
    Clockify,
    Directus,
};
use crate::models::metrics_clockify;


#[get("/metrics")]
pub async fn index(
        directus: &State<Directus>
) -> &'static str {
    let r = metrics_clockify::MetricsClockify::list(directus).await;
    "im the metrics"
}


#[get("/metrics/cron/clockify")]
pub async fn clockfyCron(
        clockify: &State<Clockify>,
        directus: &State<Directus>
        ) -> &'static str {
    println!("{}", &clockify.token.to_string());
    let r = metrics_clockify::MetricsClockify::get_info(clockify, directus).await;
    "im the clockify cron"
}