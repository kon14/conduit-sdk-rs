use super::proto::HealthCheckRequest;
use super::{HealthCheckResponse, HealthClient};
use crate::error::ConduitSdkError;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tonic::Streaming;

impl HealthClient {
    pub async fn watch(&self, service: String) -> Result<HealthWatchStream, ConduitSdkError> {
        let req = HealthCheckRequest { service };
        let proto_stream = self._client.clone().watch(req).await?.into_inner();
        let sdk_stream = HealthWatchStream::new(proto_stream);
        Ok(sdk_stream)
    }
}

pub struct HealthWatchStream {
    inner: Streaming<super::proto::HealthCheckResponse>,
}

impl HealthWatchStream {
    pub fn new(inner: Streaming<super::proto::HealthCheckResponse>) -> Self {
        HealthWatchStream { inner }
    }
}

impl Stream for HealthWatchStream {
    type Item = Result<HealthCheckResponse, ConduitSdkError>;

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
