


use super::HtmlResponse;
use rocket_dyn_templates::{context, Template};

#[get("/", format = "text/html")]
pub async fn homepage() -> HtmlResponse {

    let context = context! {};

    Ok(Template::render("index", &context))
}


#[get("/do_health_check")]
pub async fn health_check() -> &'static str {
    ""
}
