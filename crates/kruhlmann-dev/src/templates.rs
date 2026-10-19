use askama::Template;

use crate::TexContext;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexHtmlTemplate {
    pub tex: TexContext,
}
