use askama::Template;

#[derive(Template)]
#[template(path = "./authentication.html")]
pub struct AuthenticationTemplate<'a> {
    pub token: &'a str,
    pub location: &'a str,
}

#[derive(Template)]
#[template(path = "./authentication/login.html")]
pub struct AuthenticationLoginTemplate<'a> {
    pub token: &'a str,
    pub error: bool,
}

#[derive(Template)]
#[template(path = "./authentication/form_name.html")]
pub struct AuthenticationFormNameTemplate<'a> {
    pub token: &'a str,
    pub value: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "./authentication/form_password.html")]
pub struct AuthenticationFormPasswordTemplate<'a> {
    pub token: &'a str,
    pub value: &'a str,
    pub error: Option<&'a str>,
}
