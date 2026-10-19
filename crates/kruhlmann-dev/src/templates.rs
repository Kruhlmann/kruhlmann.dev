use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::TexContext;

pub struct HtmlTemplate<T>(pub T);

pub fn template_response(rendered: Result<String, askama::Error>) -> Response {
    match rendered {
        Ok(html) => Html(html).into_response(),
        Err(error) => {
            tracing::error!(?error, "failed to render template");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> Response {
        template_response(self.0.render())
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexHtmlTemplate {
    pub tex: TexContext,
}
