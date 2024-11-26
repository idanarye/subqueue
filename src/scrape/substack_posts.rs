use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::Deserialize;

use super::PagedFetcher;

pub struct BlogPostFetcher {
    api_url: Url,
}

impl BlogPostFetcher {
    pub fn new(blog_url: &str) -> anyhow::Result<Self> {
        let mut api_url = Url::parse(blog_url)?.join("api/v1/archive")?;
        api_url.query_pairs_mut().append_pair("sort", "new");
        Ok(Self { api_url })
    }
}

impl PagedFetcher for BlogPostFetcher {
    type Item = BlogPost;
    type Key = usize;

    fn page_size(&self) -> usize {
        12
    }

    fn extract_key(item: &Self::Item) -> Self::Key {
        item.id
    }

    async fn fetch(&self, offset: usize, limit: usize) -> anyhow::Result<Vec<BlogPost>> {
        let mut url = self.api_url.clone();
        url.query_pairs_mut()
            .append_pair("offset", &format!("{offset}"))
            .append_pair("limit", &format!("{limit}"));
        let result = reqwest::get(url).await?;
        Ok(serde_json::from_slice(&result.bytes().await?)?)
    }
}

#[derive(Deserialize, Debug)]
pub struct BlogPost {
    pub id: usize,
    pub title: String,
    pub post_date: DateTime<Utc>,
    pub canonical_url: String,
}
