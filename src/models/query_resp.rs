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
    #[serde(rename = "company_name")]
    pub company_name: String,
    #[serde(rename = "date_updated")]
    pub date_updated: String,
    #[serde(rename = "date_created")]
    pub date_created: String,
    pub tags: Vec<JobTag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobTag {
    #[serde(rename = "tag_id")]
    pub tag_id: TagId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagId {
    pub name: String,
    pub id: String,
}




