use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};

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

    fn are_same(item1: &Self::Item, item2: &Self::Item) -> bool {
        item1.id == item2.id
    }

    async fn fetch(&self, offset: usize, limit: usize) -> anyhow::Result<Vec<BlogPost>> {
        let mut url = self.api_url.clone();
        url.query_pairs_mut()
            .append_pair("offset", &format!("{offset}"))
            .append_pair("limit", &format!("{limit}"));
        let mut result = reqwest::get(url.clone()).await?;
        for backoff in [
            Duration::from_secs(1),
            Duration::from_secs(5),
            Duration::from_secs(30),
            Duration::from_secs(120),
        ] {
            if result.status() == StatusCode::TOO_MANY_REQUESTS {
                tracing::info!("Got {} - Backing off for {:?}", result.status(), backoff);
                tokio::time::sleep(backoff).await;
                result = reqwest::get(url.clone()).await?;
            } else {
                break;
            }
        }
        if result.status().is_success() {
            Ok(serde_json::from_slice(&result.bytes().await?)?)
        } else {
            anyhow::bail!(
                "Failed with error {}.\n{}",
                result.status(),
                result.text().await?
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPost {
    pub id: usize,
    pub title: String,
    pub post_date: DateTime<Utc>,
    pub canonical_url: String,
}
