use askama::Template;

#[derive(Template)]
#[template(path = "./toast.html")]
pub struct ToastTemplate<'a> {
    pub content: &'a str,
}
