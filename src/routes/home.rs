


use super::HtmlResponse;
use rocket_dyn_templates::{context, Template};

#[get("/", format = "text/html")]
pub fn homepage() -> HtmlResponse {

    let context = context! {};

    Ok(Template::render("index", &context))
}