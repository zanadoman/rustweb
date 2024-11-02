use askama::Template;

use crate::models::message::MessageModel;

#[derive(Template)]
#[template(path = "./message/show.html")]
pub struct MessageShowTemplate<'a> {
    pub token: &'a str,
    pub message: &'a MessageModel,
}

#[derive(Template)]
#[template(path = "./message/index.html")]
pub struct MessageIndexTemplate<'a> {
    pub token: &'a str,
    pub messages: &'a Vec<MessageModel>,
}

#[derive(Template)]
#[template(path = "./message/event.html")]
pub struct MessageEventTemplate<'a> {
    pub token: &'a str,
    pub message: &'a MessageModel,
}

#[derive(Template)]
#[template(path = "./message/form_title.html")]
pub struct MessageFormTitleTemplate<'a> {
    pub token: &'a str,
    pub value: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "./message/form_content.html")]
pub struct MessageFormContentTemplate<'a> {
    pub token: &'a str,
    pub value: &'a str,
    pub error: Option<&'a str>,
}
