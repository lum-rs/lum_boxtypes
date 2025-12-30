use std::{error::Error, future::Future, pin::Pin};

pub type BoxedError = Box<dyn Error + Send + Sync>;
pub type BoxedErrorResult<T> = Result<T, BoxedError>;

pub type PinnedBoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
pub type PinnedBoxedFutureResult<T> = PinnedBoxedFuture<BoxedErrorResult<T>>;

pub type PinnedBoxedSyncFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
pub type PinnedBoxedSyncFutureResult<T> = PinnedBoxedSyncFuture<BoxedErrorResult<T>>;

pub type LifetimedPinnedBoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type LifetimedPinnedBoxedFutureResult<'a, T> =
    LifetimedPinnedBoxedFuture<'a, BoxedErrorResult<T>>;

pub type LifetimedPinnedBoxedSyncFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;
pub type LifetimedPinnedBoxedSyncFutureResult<'a, T> =
    LifetimedPinnedBoxedSyncFuture<'a, BoxedErrorResult<T>>;
