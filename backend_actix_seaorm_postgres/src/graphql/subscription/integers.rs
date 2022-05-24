use async_graphql::Subscription;
use core::time::Duration;
use entity::async_graphql;
use futures_util::Stream;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct IntegersSubscription;

#[Subscription]
impl IntegersSubscription {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }
}
