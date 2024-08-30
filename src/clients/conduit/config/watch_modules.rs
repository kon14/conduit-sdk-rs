use super::ConfigClient;
use crate::error::ConduitSdkError;
use crate::sd::ServiceDiscoveryState;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tonic::Streaming;

impl ConfigClient {
    pub async fn watch_modules(&self) -> Result<WatchModulesStream, ConduitSdkError> {
        let req = tonic::Request::new(());
        let proto_stream = self._client.clone().watch_modules(req).await?.into_inner();
        let sdk_stream = WatchModulesStream::new(proto_stream);
        Ok(sdk_stream)
    }
}

pub struct WatchModulesStream {
    inner: Streaming<super::proto::ModuleListResponse>,
}

impl WatchModulesStream {
    pub fn new(inner: Streaming<super::proto::ModuleListResponse>) -> Self {
        WatchModulesStream { inner }
    }
}

impl Stream for WatchModulesStream {
    type Item = Result<ServiceDiscoveryState, ConduitSdkError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let inner = Pin::new(&mut self.get_mut().inner);
        match inner.poll_next(cx) {
            Poll::Ready(Some(result)) => Poll::Ready(Some(
                result.map(|proto_response| proto_response.try_into())?,
            )),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
