use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobQueryResults {
    pub data: Vec<JobResultItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobResultItem {
    pub id: String,
    pub status: String,
    pub user_created: String,
    pub date_created: String,
    pub user_updated: String,
    pub date_updated: String,
    pub company_name: String,
    pub company_url: Option<String>,
    pub position: String,
    pub position_type: String,
    pub category: String,
    pub min_per_year: i64,
    pub max_per_year: i64,
    pub description: String,
    pub how_to_apply: String,
    pub apply_url: String,
    pub apply_email: String,
    pub publish_date: String,
    pub expires_date: String,
    pub slug: String,
    pub gen_id: i64,
    pub tags: Option<Vec<Tag>>,
    pub region: Option<Vec<Region>>,
    pub countries: Option<Vec<Country>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub tag_id: TagId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagId {
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub region_id: RegionId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionId {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub country_id: CountryId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryId {
    pub id: i64,
    pub name: String,
}
