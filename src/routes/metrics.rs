
use rocket::State;
use crate::Clockify;
use crate::models::metrics_clockify;

#[get("/metrics/cron/clockify")]
pub async fn clockfyCron(clockify: &State<Clockify>) -> &'static str {
    println!("{}", &clockify.token.to_string());
    let r = metrics_clockify::MetricsClockify::get_info(clockify).await;
    "im the clockify cron"
}