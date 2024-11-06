use askama::Template;

#[derive(Template)]
#[template(path = "./toast.html")]
pub struct ToastTemplate<'a> {
    message: &'a str,
}

impl<'a> ToastTemplate<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
