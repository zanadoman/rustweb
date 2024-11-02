use askama::Template;

#[derive(Template)]
#[template(path = "./authentication.html")]
pub struct AuthenticationTemplate<'a> {
    pub token: &'a str,
    pub location: &'a str,
}
