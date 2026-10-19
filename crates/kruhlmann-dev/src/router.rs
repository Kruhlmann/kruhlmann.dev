use axum::extract::State;

use crate::{AppState, HtmlTemplate, IndexHtmlTemplate};

pub async fn index(State(state): State<AppState>) -> HtmlTemplate<IndexHtmlTemplate> {
    HtmlTemplate(IndexHtmlTemplate { tex: state.tex })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{TexContext, TexContextEducation, TexContextExperience};

    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn index_handler_returns_ok_html_response() {
        let uuid = "3cf1c79b-9eb7-4ec6-8530-e50c8e6f115e".to_string();
        let state = AppState {
            tex: TexContext {
                education: vec![TexContextEducation {
                    graduation_year: 0,
                    institution: uuid.clone(),
                    location: String::new(),
                    title: String::new(),
                }],
                experience: vec![TexContextExperience {
                    year_start: 0,
                    year_stop: 0,
                    position: String::new(),
                    company: String::new(),
                    assignments: HashMap::new(),
                }],
                languages: HashMap::new(),
                methods: Vec::new(),
                tools: HashMap::new(),
            },
        };
        let response = index(State(state)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| v.contains(&uuid))
        );
    }
}
