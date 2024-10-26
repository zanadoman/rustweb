use askama::Template;

#[derive(Template)]
#[template(path = "authentication.html")]
pub struct AuthenticationTemplate<'a> {
    pub token: String,
    pub location: &'a str,
}
