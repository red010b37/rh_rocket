use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use crate::errors::our_error::OurError;

use crate::services::{countries_svc, region_svc, tags_svc};
use crate::services::countries_svc::Country;
use crate::services::region_svc::Region;
use crate::services::tags_svc::Tag;
use crate::states::AppSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataResults {
    pub tags: Vec<Tag>,
    pub regions: Vec<Region>,
    pub countries: Vec<Country>,
}


pub async fn execute<'r>(
    app_settings: &State<AppSettings>,
) -> Result<DataResults, OurError> {

    let tags = tags_svc::get_all(app_settings).await?;
    let regions =  region_svc::get_all(app_settings).await?;
    let countries =  countries_svc::get_all(app_settings).await?;

    Ok(DataResults {
        tags,
        regions,
        countries
    })
}
