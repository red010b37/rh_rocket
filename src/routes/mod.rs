use rocket::http::Status;
use rocket_dyn_templates::Template;

type HtmlResponse = Result<Template, Status>;