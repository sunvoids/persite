use askama::Template;

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate<'a> {
    pub title: &'a str,
    pub content: String,
    pub categories: &'a [Category]
}

pub struct Category {
    pub name: String,
    pub slug: String,
}