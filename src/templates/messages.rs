use askama::Template;

use crate::models::message::MessageModel;

#[derive(Template)]
#[template(path = "messages.html")]
pub struct MessagesTemplate<'a> {
    pub messages: &'a Vec<MessageModel>,
}
