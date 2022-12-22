use rocket::serde::{Deserialize, Serialize};

// GET JOB RESULT
// ------------------------------------------------------------------------------------------------
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetJobsResult {
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub id: String,
    pub status: String,
    pub position: String,
    #[serde(rename = "company_name")]
    pub company_name: String,
    #[serde(rename = "min_per_year")]
    pub min_per_year: i64,
    #[serde(rename = "max_per_year")]
    pub max_per_year: i64,
    pub slug: String,
    #[serde(rename = "date_updated")]
    pub date_updated: String,
    #[serde(rename = "date_created")]
    pub date_created: String,
    pub tags: Vec<TagR>,
    pub region: Vec<Region>,
    pub countries: Vec<Country>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagR {
    #[serde(rename = "tag_id")]
    pub tag_id: TagId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagId {
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    #[serde(rename = "region_id")]
    pub region_id: RegionId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionId {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    #[serde(rename = "country_id")]
    pub country_id: CountryId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryId {
    pub id: i64,
    pub name: String,
}
