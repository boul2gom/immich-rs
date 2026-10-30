use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;

use crate::error::Result;

/// A concrete, [`Unpin`] asynchronous stream yielding paginated items or pages.
///
/// Wraps an underlying boxed stream, implementing [`Stream`] while also providing inherent
/// [`.next()`](Self::next), [`.try_next()`](Self::try_next), and [`.collect()`](Self::collect)
/// asynchronous methods so callers do not need to import [`futures_util::StreamExt`] or
/// use [`std::pin::pin!`].
///
/// # Examples
///
/// Consuming items one by one without importing stream traits:
///
/// ```no_run
/// # async fn doc() -> Result<(), immich_rs::Error> {
/// # let client: immich_rs::ImmichClient = unimplemented!();
/// let mut people = client.people().get_all_people().into_paginator().items();
/// while let Some(person) = people.try_next().await? {
///     println!("Person: {:?}", person.name);
/// }
/// # Ok(())
/// # }
/// ```
///
/// Collecting all items into a single vector:
///
/// ```no_run
/// # async fn doc() -> Result<(), immich_rs::Error> {
/// # let client: immich_rs::ImmichClient = unimplemented!();
/// let all_people = client
///     .people()
///     .get_all_people()
///     .into_paginator()
///     .items()
///     .collect()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[must_use = "streams do nothing unless polled or awaited"]
pub struct PaginationStream<'a, T> {
    inner: crate::BoxStream<'a, Result<T>>,
}

impl<'a, T> PaginationStream<'a, T> {
    /// Creates a new `PaginationStream` wrapping the given stream.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<T>> + Send + 'a,
    {
        Self {
            inner: Box::pin(stream),
        }
    }

    /// Creates a new `PaginationStream` wrapping the given stream.
    #[cfg(target_arch = "wasm32")]
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<T>> + 'a,
    {
        Self {
            inner: Box::pin(stream),
        }
    }

    /// Yields the next item in the stream, or `None` if the stream is exhausted.
    ///
    /// This method allows consuming the stream sequentially without requiring an import
    /// of [`futures_util::StreamExt`].
    ///
    /// # Errors
    ///
    /// Returns `Some(Err(...))` if fetching the next page of items fails.
    pub async fn next(&mut self) -> Option<Result<T>> {
        use futures_util::StreamExt;
        self.inner.next().await
    }

    /// Yields the next item in the stream, unwrapping the inner [`Option`] and [`Result`].
    ///
    /// This method allows consuming the stream with idiomatic `while let Some(item) = stream.try_next().await?`
    /// loops without requiring an import of [`futures_util::TryStreamExt`].
    ///
    /// # Errors
    ///
    /// Returns `Err(...)` if fetching the next page of items fails.
    pub async fn try_next(&mut self) -> Result<Option<T>> {
        self.next().await.transpose()
    }

    /// Consumes the entire stream and collects all items into a [`Vec<T>`].
    ///
    /// Stops and returns an error immediately if any paginated request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if any page request in the stream fails.
    pub async fn collect(mut self) -> Result<Vec<T>> {
        let mut items = Vec::new();
        while let Some(res) = self.next().await {
            items.push(res?);
        }
        Ok(items)
    }

    /// Alias for [`Self::collect`]. Consumes the entire stream and collects all items into a [`Vec<T>`].
    ///
    /// # Errors
    ///
    /// Returns an error if any page request in the stream fails.
    pub async fn try_collect(self) -> Result<Vec<T>> {
        self.collect().await
    }

    /// Unwraps this `PaginationStream` into its underlying pinned boxed stream.
    pub fn into_inner(self) -> crate::BoxStream<'a, Result<T>> {
        self.inner
    }
}

impl<T> Stream for PaginationStream<'_, T> {
    type Item = Result<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

impl<T> std::fmt::Debug for PaginationStream<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaginationStream").finish_non_exhaustive()
    }
}
