use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate<'a> {
    pub location: &'a str,
    pub username: &'a str,
}
