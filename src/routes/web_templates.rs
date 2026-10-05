use askama::Template;
use wyrite::WebError;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct Post<'a> {
    pub post: &'a wyrite::Post,
}

#[derive(Template)]
#[template(path = "edit_post.html")]
pub struct EditPost<'a> {
    pub post: &'a Option<wyrite::Post>,
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct Posts<'a> {
    pub posts: &'a Vec<wyrite::Post>,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct Error<'a> {
    pub error: &'a WebError,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct NotFound<'a> {
    pub error: &'a String,
}
