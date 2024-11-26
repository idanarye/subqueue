use std::future::Future;

pub mod substack_posts;

pub trait PagedFetcher {
    type Item;
    type Key: core::fmt::Debug + PartialEq;

    fn fetch(
        &self,
        offset: usize,
        limit: usize,
    ) -> impl Future<Output = anyhow::Result<Vec<Self::Item>>>;

    fn extract_key(item: &Self::Item) -> Self::Key;

    fn are_same(item1: &Self::Item, item2: &Self::Item) -> bool;

    fn page_size(&self) -> usize;

    fn find_num_items(&self) -> impl Future<Output = anyhow::Result<usize>> {
        async {
            let mut num = 1;
            const ADVANCE_BY: usize = 8;
            let (mut small, mut big) = loop {
                if self.fetch(num, 1).await?.is_empty() {
                    break (num >> ADVANCE_BY, num);
                }
                num <<= ADVANCE_BY;
            };

            while small + 1 < big {
                let mid = (small + big) / 2;

                if self.fetch(mid, 1).await?.is_empty() {
                    big = mid;
                } else {
                    small = mid;
                }
            }

            // Big is the lowest number we can't get any from - which means it's the number of
            // items.
            Ok(big)
        }
    }

    fn fetch_all(&self) -> impl Future<Output = anyhow::Result<Vec<Self::Item>>> {
        async {
            let mut collected_items = Vec::new();

            loop {
                let batch = self.fetch(collected_items.len().max(1) - 1, 0).await?;
                let mut batch_it = batch.into_iter();
                let Some(first) = batch_it.next() else {
                    break;
                };
                if let Some(prev_last) = collected_items.last() {
                    if !Self::are_same(prev_last, &first) {
                        // TODO: make it a sepcial, actionable error
                        anyhow::bail!("Different items");
                    }
                } else {
                    // First time - just add them all
                    collected_items.push(first);
                }
                if let Some(second) = batch_it.next() {
                    collected_items.push(second);
                } else {
                    break;
                }
                collected_items.extend(batch_it);
                tracing::debug!("Got {} items so far", collected_items.len());
            }

            collected_items.reverse();
            Ok(collected_items)
        }
    }
}
