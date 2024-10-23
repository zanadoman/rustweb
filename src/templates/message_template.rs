use askama::Template;

use crate::models::message_model::MessageModel;

#[derive(Template)]
#[template(path = "message.html")]
pub struct MessageTemplate<'a> {
    pub message: &'a MessageModel,
}
