#![cfg(feature = "tokio")]

use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::Duration;
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, MutexGuard};

pub static GLOBAL_TOKIO_TIMEOUT: Lazy<Arc<AtomicU64>> = Lazy::new(|| Arc::new(AtomicU64::new(5)));

/// A wrapper around `tokio::sync::Mutex` that allows for a timeout to be set.
pub struct MutexWithTimeout<T> {
    inner: Mutex<T>,
    timeout: Duration,
}

impl<T> MutexWithTimeout<T> {
    /// Creates a new `MutexWithTimeout` with the default timeout.
    pub fn new(inner: T) -> Self {
        Self {
            inner: Mutex::new(inner),
            timeout: Duration::from_secs(GLOBAL_TOKIO_TIMEOUT.load(std::sync::atomic::Ordering::Relaxed)),
        }
    }

    /// Creates a new `MutexWithTimeout` with the given timeout.
    pub async fn new_with_timeout(inner: T, timeout: Duration) -> Self {
        Self {
            inner: Mutex::new(inner),
            timeout,
        }
    }

    /// Will attempt to lock the inner `std::sync::Mutex`.
    /// If the lock is not acquired within the timeout, `None` will be returned.
    pub async fn lock(&self) -> Option<MutexGuard<T>> {
        let start = std::time::Instant::now();
        loop {
            if let Ok(guard) = self.inner.try_lock() {
                return Some(guard);
            }

            if start.elapsed() > self.timeout {
                break;
            }
        }
        None
    }

    /// Calls the inner mutex's `try_lock` method.
    pub async fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.inner.try_lock().ok()
    }
}

/// A version of `MutexWithTimeout` that will automatically panic if the lock is not acquired within the timeout.
/// Acts as a drop-in replacement for `tokio::sync::Mutex`.
pub struct MutexWithTimeoutAuto<T> {
    inner: Mutex<T>,
    timeout: Duration,
}

impl<T> MutexWithTimeoutAuto<T> {
    /// Creates a new `MutexWithTimeoutAuto` with the default timeout.
    pub fn new(inner: T) -> Self {
        Self {
            inner: Mutex::new(inner),
            timeout: Duration::from_secs(GLOBAL_TOKIO_TIMEOUT.load(std::sync::atomic::Ordering::Relaxed)),
        }
    }

    /// Creates a new `MutexWithTimeoutAuto` with the given timeout.
    pub async fn new_with_timeout(inner: T, timeout: Duration) -> Self {
        Self {
            inner: Mutex::new(inner),
            timeout,
        }
    }

    /// Will attempt to lock the inner `std::sync::Mutex`.
    /// If the lock is not acquired within the timeout, the thread will panic.
    pub async fn lock(&self) -> MutexGuard<T> {
        let start = std::time::Instant::now();
        loop {
            if let Ok(guard) = self.inner.try_lock() {
                return guard;
            }

            if start.elapsed() > self.timeout {
                break;
            }
        }
        panic!("MutexWithTimeoutAuto lock timed out after {:?}", self.timeout);
    }

    /// Calls the inner mutex's `try_lock` method.
    pub async fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.inner.try_lock().ok()
    }
}