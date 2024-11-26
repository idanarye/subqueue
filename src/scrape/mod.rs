use std::future::Future;

pub mod substack_posts;

pub trait PagedFetcher {
    type Item;
    type Key;

    fn fetch(&self, offset: usize, limit: usize) -> impl Future<Output = anyhow::Result<Vec<Self::Item>>>;

    fn extract_key(item: &Self::Item) -> Self::Key;

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
}
