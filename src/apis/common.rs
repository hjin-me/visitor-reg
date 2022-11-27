use askama::Template;

#[derive(Template)]
#[template(path = "forbidden.html")]
pub struct ForbiddenTemplate {}

