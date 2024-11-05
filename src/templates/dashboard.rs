use askama::Template;

use super::message::{MessageFormContentTemplate, MessageFormTitleTemplate};

#[derive(Template)]
#[template(path = "./dashboard.html")]
pub struct DashboardTemplate<'a> {
    pub token: &'a str,
    pub location: &'a str,
    pub name: Option<&'a str>,
    pub message_form_title: &'a MessageFormTitleTemplate<'a>,
    pub message_form_content: &'a MessageFormContentTemplate<'a>,
}
