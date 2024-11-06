use askama::Template;

use crate::models::message::MessageModel;

#[derive(Template)]
#[template(path = "./message/show.html")]
pub struct MessageShowTemplate<'a> {
    token: &'a str,
    id: i32,
    form_title: MessageFormTitleTemplate<'a>,
    form_content: MessageFormContentTemplate<'a>,
}

impl<'a> MessageShowTemplate<'a> {
    pub fn new(
        token: &'a str,
        id: i32,
        title: &'a str,
        content: &'a str,
    ) -> Self {
        Self {
            token,
            id,
            form_title: MessageFormTitleTemplate::new(token, id, title),
            form_content: MessageFormContentTemplate::new(token, id, content),
        }
    }
}

#[derive(Template)]
#[template(path = "./message/index.html")]
pub struct MessageIndexTemplate<'a> {
    token: &'a str,
    messages: &'a Vec<MessageModel>,
}

impl<'a> MessageIndexTemplate<'a> {
    pub fn new(token: &'a str, messages: &'a Vec<MessageModel>) -> Self {
        Self { token, messages }
    }
}

#[derive(Template)]
#[template(path = "./message/event.html")]
pub struct MessageEventTemplate<'a> {
    token: &'a str,
    message: &'a MessageModel,
}

impl<'a> MessageEventTemplate<'a> {
    pub fn new(token: &'a str, message: &'a MessageModel) -> Self {
        Self { token, message }
    }
}

#[derive(Template)]
#[template(path = "./message/form_title.html")]
pub struct MessageFormTitleTemplate<'a> {
    token: &'a str,
    id: i32,
    value: &'a str,
    error: Option<&'a str>,
}

impl<'a> MessageFormTitleTemplate<'a> {
    pub fn new(token: &'a str, id: i32, value: &'a str) -> Self {
        Self {
            token,
            id,
            value,
            error: None,
        }
    }

    pub fn validate(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }
}

#[derive(Template)]
#[template(path = "./message/form_content.html")]
pub struct MessageFormContentTemplate<'a> {
    token: &'a str,
    id: i32,
    value: &'a str,
    error: Option<&'a str>,
}

impl<'a> MessageFormContentTemplate<'a> {
    pub fn new(token: &'a str, id: i32, value: &'a str) -> Self {
        Self {
            token,
            id,
            value,
            error: None,
        }
    }

    pub fn validate(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }
}
