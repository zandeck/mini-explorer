use std::hash::{Hash, Hasher};

use futures::stream::BoxStream;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite::http::Uri;

use crate::chain::ChainEvent;

pub fn progress(uri: Uri, s: Option<ReceiverStream<ChainEvent>>) -> iced::Subscription<ChainEvent> {
    iced::Subscription::from_recipe(SyncProgressEngine { uri, s })
}

#[derive(Debug, Default)]
pub struct SyncProgressEngine {
    pub s: Option<ReceiverStream<ChainEvent>>,
    pub uri: Uri,
}

impl SyncProgressEngine {
    pub fn new(uri: Uri, s: Option<ReceiverStream<ChainEvent>>) -> Self {
        Self { s, uri }
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for SyncProgressEngine
where
    H: Hasher,
{
    type Output = ChainEvent;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.uri.hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<I>) -> BoxStream<Self::Output> {
        Box::pin(self.s.expect("something went wrong").map(move |e| e))
    }
}
