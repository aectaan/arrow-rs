// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! [`FallibleRequestStream`] and [`FallibleTonicResponseStream`] adapters

use futures::{Stream, StreamExt, channel::oneshot::Sender};
use std::pin::Pin;
use std::task::{Poll, ready};

/// Wrapper around a fallible stream (one that returns errors) that makes it infallible.
///
/// Any errors encountered in the stream are ignored are sent to the provided
/// oneshot sender.
///
/// This can be used to accept a stream of `Result<_>` from a client API and send
/// them to the remote server that wants only the successful results.
pub struct FallibleRequestStream<T, E> {
    /// sender to notify error
    sender: Option<Sender<E>>,
    /// fallible stream
    fallible_stream: Pin<Box<dyn Stream<Item = std::result::Result<T, E>> + Send + 'static>>,
}

impl<T, E> FallibleRequestStream<T, E> {
    /// Create a FallibleRequestStream
    pub fn new(
        sender: Sender<E>,
        fallible_stream: Pin<Box<dyn Stream<Item = std::result::Result<T, E>> + Send + 'static>>,
    ) -> Self {
        Self {
            sender: Some(sender),
            fallible_stream,
        }
    }
}

impl<T, E> Stream for FallibleRequestStream<T, E> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let pinned = self.get_mut();
        let mut request_streams = pinned.fallible_stream.as_mut();
        match ready!(request_streams.poll_next_unpin(cx)) {
            Some(Ok(data)) => Poll::Ready(Some(data)),
            Some(Err(e)) => {
                // in theory this should only ever be called once
                // as this stream should not be polled again after returning
                // None, however we still check for None to be safe
                if let Some(sender) = pinned.sender.take() {
                    // an error means the other end of the channel is not around
                    // to receive the error, so ignore it
                    let _ = sender.send(e);
                }
                Poll::Ready(None)
            }
            None => Poll::Ready(None),
        }
    }
}
