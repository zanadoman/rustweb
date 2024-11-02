use askama::Template;

use crate::models::message::MessageModel;

#[derive(Template)]
#[template(path = "./dashboard.html")]
pub struct DashboardTemplate<'a> {
    pub token: &'a str,
    pub location: &'a str,
    pub username: &'a str,
    pub messages: &'a Vec<MessageModel>,
}
