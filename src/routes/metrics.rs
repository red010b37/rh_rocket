use serde::{Deserialize, Serialize};

use super::HtmlResponse;
use crate::models::metrics_clockify;
use crate::states::{Clockify, Directus};
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/metrics")]
pub async fn index(directus: &State<Directus>) -> HtmlResponse {
    let r = metrics_clockify::MetricsClockify::list(directus).await;

    let mut d = r.unwrap().data;

    d.sort_by(|a, b| a.log_date.cmp(&b.log_date));

    let mut gdata: Vec<GraphData> = vec![];

    for item in &d {
        let log_date = item.log_date.to_string();

        let mut exists = false;
        for di in gdata.iter_mut() {
            if di.date == log_date {
                exists = true;
                di.amount_cents -= item.amount.clone();
                di.duration_seconds += item.duration.clone();
            }
        }

        if !exists {
            gdata.push(GraphData {
                amount_cents: item.amount.clone() * -1,
                date: log_date.clone(),
                duration_seconds: item.duration.clone(),
                cumulative_amount_cents: 0.0,
            })
        }
    }

    let mut ct = 0.0;
    for di in gdata.iter_mut() {
        ct += di.amount_cents.clone() as f32 / 100.0;
        di.cumulative_amount_cents = ct.clone();
    }

    Ok(Template::render(
        "metrics/metrics",
        context! {
            clockifydata: &gdata,
            cumulative_total: (ct.round() * -1.0),
        },
    ))
}

#[derive(Serialize, Deserialize)]
pub struct GraphData {
    pub date: String,
    pub amount_cents: i32,
    pub duration_seconds: i32,
    pub cumulative_amount_cents: f32,
}

#[get("/metrics/cron/clockify")]
pub async fn clockfyCron(clockify: &State<Clockify>, directus: &State<Directus>) -> &'static str {
    println!("{}", &clockify.token.to_string());
    let r = metrics_clockify::MetricsClockify::get_info(clockify, directus).await;
    "im the clockify cron"
}
