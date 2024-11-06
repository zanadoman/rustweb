use askama::Template;

use super::message::{MessageFormContentTemplate, MessageFormTitleTemplate};

#[derive(Template)]
#[template(path = "./dashboard.html")]
pub struct DashboardTemplate<'a> {
    token: &'a str,
    location: &'a str,
    name: Option<&'a str>,
    message_form_title: MessageFormTitleTemplate<'a>,
    message_form_content: MessageFormContentTemplate<'a>,
}

impl<'a> DashboardTemplate<'a> {
    pub fn new(token: &'a str, name: &'a str) -> Self {
        Self {
            token,
            location: "Dashboard",
            name: Some(name),
            message_form_title: MessageFormTitleTemplate::new(token, 0, ""),
            message_form_content: MessageFormContentTemplate::new(token, 0, ""),
        }
    }
}
