use std::{error::Error, future::Future, pin::Pin};

pub type BoxedError = Box<dyn Error + Send + Sync>;
pub type BoxedErrorResult<T> = Result<T, BoxedError>;

pub type PinnedBoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
pub type PinnedBoxedFutureResult<T> = PinnedBoxedFuture<BoxedErrorResult<T>>;

pub type LifetimedPinnedBoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;
pub type LifetimedPinnedBoxedFutureResult<'a, T> =
    LifetimedPinnedBoxedFuture<'a, BoxedErrorResult<T>>;
