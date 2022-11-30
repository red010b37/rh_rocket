use rocket::http::Status;
use rocket_dyn_templates::Template;

pub mod jobs;

type HtmlResponse = Result<Template, Status>;