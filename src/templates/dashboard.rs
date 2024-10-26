use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate<'a> {
    pub token: &'a str,
    pub location: &'a str,
    pub username: &'a str,
}
