use crate::{PageCache, TexContext};

#[derive(Clone)]
pub struct AppState {
    pub tex: TexContext,
    pub page_cache: PageCache,
}
