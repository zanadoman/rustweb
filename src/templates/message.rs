use askama::Template;

use crate::models::message::MessageModel;

#[derive(Template)]
#[template(path = "message.html")]
pub struct MessageTemplate<'a> {
    pub token: &'a str,
    pub message: &'a MessageModel,
}
