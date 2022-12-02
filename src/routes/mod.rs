use rocket::http::Status;
use rocket_dyn_templates::Template;

pub mod jobs;
pub mod metrics;

type HtmlResponse = Result<Template, Status>;