#[derive(Debug, Clone)]
pub struct Directus {
    pub token: String,
    pub directus_api_url: String,
}

#[derive(Debug, Clone)]
pub struct Clockify {
    pub token: String,
}

pub struct AppSettings {
    pub env: String,
    pub clockify: Clockify,
    pub directus: Directus,
}
