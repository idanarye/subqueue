use std::future::Future;

use futures::future::join_all;
use itertools::Itertools;

pub mod substack_posts;

pub trait PagedFetcher {
    type Item;
    type Key: core::fmt::Debug + PartialEq;

    fn fetch(&self, offset: usize, limit: usize) -> impl Future<Output = anyhow::Result<Vec<Self::Item>>>;

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
            let page_size = self.page_size();
            let num_items = self.find_num_items().await?;


            let mut collected_items = Vec::new();
            collected_items.reserve_exact(num_items);

            let starting_points_iteartor = (0..num_items).step_by(page_size - 1);
            for starting_points_batch in starting_points_iteartor.chunks(1).into_iter() {
                // let starting_points_batch = starting_points_batch.collect_vec();
                for batch in join_all(starting_points_batch.map(|i| self.fetch(i, page_size))).await {
                    if let Some(prev_last) = collected_items.last() {
                        let mut it = batch?.into_iter();
                        let Some(first) = it.next() else {
                            anyhow::bail!("empty result");
                        };
                        if !Self::are_same(prev_last, &first) {
                            // TODO: make it a sepcial, actionable error
                            anyhow::bail!("Different items");
                        }
                        collected_items.extend(it);
                    } else {
                        // First time - just add them all
                        collected_items.extend(batch?);
                    }
                    tracing::debug!("Got {} items so far", collected_items.len());
                }
            }

            collected_items.reverse();
            Ok(collected_items)
        }
    }
}
