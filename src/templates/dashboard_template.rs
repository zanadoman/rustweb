use askama::Template;

use crate::models::message_model::MessageModel;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate<'a> {
    pub messages: &'a Vec<MessageModel>,
}
