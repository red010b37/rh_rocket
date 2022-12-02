
use rocket::State;
use crate::errors::our_error::OurError;
use crate::states::{
    Clockify,
    Directus,
};
use chrono::{Datelike, NaiveDate, Utc};
use serde::{Serialize, Deserialize};

use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;

pub struct MetricsClockify {

}

impl MetricsClockify {

    pub async fn get_info<'r>(
            clockify: &State<Clockify>,
            directus: &State<Directus>
            ) -> Result<Self, OurError> {

        let mut h = header::HeaderMap::new();
        let api_str = HeaderValue::from_str(clockify.token.as_str());
        h.insert("Accept", header::HeaderValue::from_static("application/json"));
        h.insert("X-Api-Key", api_str.unwrap());

        let lookup_date = Utc::now().naive_utc();
        let year = lookup_date.date().year();
        let month = lookup_date.month();
        let day = lookup_date.day();


        let start_date = format!("{}-{:0>2}-{:0>2}T00:00:00.000", year, month, day);
        let end_date = format!("{}-{:0>2}-{:0>2}T23:59:59.000", year, month, day);

        let reports_body = Root {
            date_range_start: start_date.to_string(),
            date_range_end: end_date.to_string(),
            summary_filter: SummaryFilter {
                groups: vec![
                "USER".to_string(),
                // "PROJECT".to_string(),
                "TIMEENTRY".to_string()
                ]
            }
        };

        let client = reqwest::Client::builder()
        .default_headers(h)
        .build()?;


        let resp = client
        .post("https://reports.api.clockify.me/v1/workspaces/628024bf6c613e21753c3a98/reports/summary")
        .json(&reports_body)
        .send()
        .await?;

        let data: ClockifyApi = resp.json().await?;
        println!("{:?}", data);

        if !data.group_one.is_empty() {
            println!("{:?}", "is running".to_string());
            let d = data.group_one.get(0).unwrap();
            for c in &d.children {
//                println!("{:?}", c.amount);

//                // TODO move this somewhere better
                let newEntryItem = NewClockifySummaryItem{
                    clockify_id: c.id.to_string(),
                    duration: c.duration,
                    amounts_type: c.amounts[0].type_field.to_string(),
                    amounts_value: c.amounts[0].value as i32,
                    amount: c.amount as i32,
                    name: "".to_string(),
                    log_date: NaiveDate::from_ymd(year, month, day),

                };

                let post_url = directus.directus_api_url.to_string() + "/items/metrics_clockify";
                println!("{:?}", post_url);
                println!("{:?}", directus.token.to_string());
                let resp = reqwest::Client::new()
                .post(post_url)
                .json(&newEntryItem)
                .bearer_auth(directus.token.to_string())
                .send()
                .await;

                if resp.is_err() {
                    print!("{:?}", resp.unwrap())
                }

            }
        }
        Ok(MetricsClockify{})
    }

}


#[derive(Serialize, Deserialize)]

pub struct NewClockifySummaryItem {
    pub clockify_id: String,
    pub duration: i32,
    pub amounts_type: String,
    pub amounts_value: i32,
    pub amount: i32,
    pub name: String,
    pub log_date: NaiveDate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    pub date_range_start: String,
    pub date_range_end: String,
    pub summary_filter: SummaryFilter,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SummaryFilter {
    pub groups: Vec<String>,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClockifyApi {
    pub totals: Option<Vec<Option<Total>>>,
    pub group_one: Vec<GroupOne>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Total {
    #[serde(rename = "_id")]
    pub id: String,
    pub total_time: i32,
    pub total_billable_time: i32,
    pub entries_count: i32,
    pub total_amount: f64,
    pub amounts: Vec<Amount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Amount {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GroupOne {
    pub duration: i32,
    pub amounts: Vec<Amount>,
    pub amount: f32,
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub name_lower_case: String,
    pub children: Vec<Children>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Children {
    pub duration: i32,
    pub amounts: Vec<Amount>,
    pub amount: f32,
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
}