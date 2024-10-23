use askama::Template;

#[derive(Template)]
#[template(path = "authentication.html")]
pub struct AuthenticationTemplate;
