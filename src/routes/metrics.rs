

use rocket::State;
use rocket_dyn_templates::{context, Template};
use crate::states:: {
    Clockify,
    Directus,
};
use crate::models::metrics_clockify;
use super::HtmlResponse;


#[get("/metrics")]
pub async fn index(
        directus: &State<Directus>
) -> HtmlResponse {
    let r = metrics_clockify::MetricsClockify::list(directus).await;
    Ok(Template::render("metrics/metrics", context! {}))
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
