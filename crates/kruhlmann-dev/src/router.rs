use std::time::Duration;

use axum::{body::Bytes, extract::State, http::StatusCode, response::Html};

use crate::{AppState, IndexHtmlTemplate};

pub async fn index(State(state): State<AppState>) -> Result<Html<Bytes>, StatusCode> {
    state
        .page_cache
        .insert_or_get(
            "index",
            || IndexHtmlTemplate { tex: state.tex },
            Duration::from_secs(3600),
        )
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{PageCache, TexContext, TexContextEducation, TexContextExperience};

    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn index_handler_returns_ok_html_response() {
        let uuid = "3cf1c79b-9eb7-4ec6-8530-e50c8e6f115e".to_string();
        let state = AppState {
            tex: TexContext {
                profile: String::new(),
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
                    assignments: BTreeMap::new(),
                }],
                languages: BTreeMap::new(),
                methods: Vec::new(),
                tools: BTreeMap::new(),
                vars: BTreeMap::new(),
            },
            page_cache: PageCache::default(),
        };
        let response = index(State(state)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| v.starts_with("text/html"))
        );

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains(&uuid));
    }
}
