use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use askama::Template;
use axum::body::Bytes;

struct CachedPage {
    html: Bytes,
    expires_at: Instant,
}

#[derive(Clone, Default)]
pub struct PageCache {
    pages: Arc<RwLock<HashMap<&'static str, CachedPage>>>,
}

impl PageCache {
    pub fn get(&self, key: &str) -> Option<Bytes> {
        self.pages
            .read()
            .expect("page cache lock poisoned")
            .get(key)
            .filter(|page| Instant::now() < page.expires_at)
            .map(|page| page.html.clone())
    }

    pub fn insert_or_get<T: Template>(
        &self,
        key: &'static str,
        make_template: impl FnOnce() -> T,
        ttl: Duration,
    ) -> Result<Bytes, askama::Error> {
        if let Some(html) = self.get(key) {
            return Ok(html);
        }
        tracing::warn!(?key, "page cache miss");

        let html = Bytes::from(make_template().render()?);
        let mut pages = self.pages.write().expect("page cache lock poisoned");
        if let Some(page) = pages
            .get(key)
            .filter(|page| Instant::now() < page.expires_at)
        {
            return Ok(page.html.clone());
        }

        pages.insert(
            key,
            CachedPage {
                html: html.clone(),
                expires_at: Instant::now() + ttl,
            },
        );

        Ok(html)
    }

    pub fn invalidate(&self, key: &str) {
        self.pages
            .write()
            .expect("page cache lock poisoned")
            .remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[derive(Template)]
    #[template(source = "{{ value }}", ext = "html")]
    struct TestTemplate<'a> {
        value: &'a str,
    }

    const TTL: Duration = Duration::from_secs(60);

    fn insert(cache: &PageCache, key: &'static str, value: &str) -> Bytes {
        cache
            .insert_or_get(key, || TestTemplate { value }, TTL)
            .unwrap()
    }

    fn expire(cache: &PageCache, key: &str) {
        cache
            .pages
            .write()
            .unwrap()
            .get_mut(key)
            .unwrap()
            .expires_at = Instant::now() - Duration::from_secs(1);
    }

    #[test]
    fn missing_key_returns_none() {
        assert_eq!(PageCache::default().get("missing"), None);
    }

    #[test]
    fn insert_renders_and_caches_html() {
        let cache = PageCache::default();

        let html = insert(&cache, "index", "<hello>");

        assert_eq!(html, Bytes::from_static(b"&#60;hello&#62;"));
        assert_eq!(cache.get("index"), Some(html));
    }

    #[test]
    fn cache_hit_does_not_construct_template() {
        let cache = PageCache::default();
        let original = insert(&cache, "index", "original");

        let result = cache
            .insert_or_get(
                "index",
                || -> TestTemplate<'static> {
                    panic!("template must not be constructed on a cache hit");
                },
                TTL,
            )
            .unwrap();

        assert_eq!(result, original);
    }

    #[test]
    fn expired_entry_is_not_returned() {
        let cache = PageCache::default();
        insert(&cache, "index", "old");
        expire(&cache, "index");

        assert_eq!(cache.get("index"), None);
    }

    #[test]
    fn expired_entry_is_replaced() {
        let cache = PageCache::default();
        insert(&cache, "index", "old");
        expire(&cache, "index");

        let html = insert(&cache, "index", "new");

        assert_eq!(html, Bytes::from_static(b"new"));
        assert_eq!(cache.get("index"), Some(html));
    }

    #[test]
    fn zero_ttl_returns_rendered_html_but_expires_immediately() {
        let cache = PageCache::default();

        let html = cache
            .insert_or_get("index", || TestTemplate { value: "hello" }, Duration::ZERO)
            .unwrap();

        assert_eq!(html, Bytes::from_static(b"hello"));
        assert_eq!(cache.get("index"), None);
    }

    #[test]
    fn invalidate_removes_only_the_requested_entry() {
        let cache = PageCache::default();
        insert(&cache, "index", "home");
        let about = insert(&cache, "about", "about");

        cache.invalidate("index");

        assert_eq!(cache.get("index"), None);
        assert_eq!(cache.get("about"), Some(about.clone()));

        // Invalidating an absent key is harmless.
        cache.invalidate("missing");
        assert_eq!(cache.get("about"), Some(about));
    }

    #[test]
    fn clones_share_insertions_and_invalidations() {
        let cache = PageCache::default();
        let cloned = cache.clone();

        let html = insert(&cache, "index", "hello");
        assert_eq!(cloned.get("index"), Some(html));

        cloned.invalidate("index");
        assert_eq!(cache.get("index"), None);
    }

    #[test]
    fn entry_inserted_during_template_creation_wins() {
        let cache = PageCache::default();

        let html = cache
            .insert_or_get(
                "index",
                || {
                    // Deterministically exercise the second cache check.
                    insert(&cache, "index", "winner");
                    TestTemplate { value: "loser" }
                },
                TTL,
            )
            .unwrap();

        assert_eq!(html, Bytes::from_static(b"winner"));
        assert_eq!(cache.get("index"), Some(html));
    }

    struct FailingDisplay;

    impl fmt::Display for FailingDisplay {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    #[derive(Template)]
    #[template(source = "{{ value }}", ext = "html")]
    struct FailingTemplate {
        value: FailingDisplay,
    }

    #[test]
    fn render_failure_is_returned_and_not_cached() {
        let cache = PageCache::default();

        let result = cache.insert_or_get(
            "index",
            || FailingTemplate {
                value: FailingDisplay,
            },
            TTL,
        );

        assert!(result.is_err());
        assert!(!cache.pages.read().unwrap().contains_key("index"));

        // A subsequent successful render can populate the cache.
        let html = insert(&cache, "index", "recovered");
        assert_eq!(cache.get("index"), Some(html));
    }
}
