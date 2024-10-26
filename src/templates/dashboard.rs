use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate<'a> {
    pub token: String,
    pub location: &'a str,
    pub username: &'a str,
}
