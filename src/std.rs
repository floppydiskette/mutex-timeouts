use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::AtomicU64;
use std::time::Duration;
use once_cell::sync::Lazy;

pub static GLOBAL_STD_TIMEOUT: Lazy<Arc<AtomicU64>> = Lazy::new(|| Arc::new(AtomicU64::new(5)));

/// A wrapper around `std::sync::Mutex` that allows for a timeout to be set.
pub struct MutexWithTimeout<T> {
    inner: Mutex<T>,
    timeout: Duration,
}

impl<T> MutexWithTimeout<T> {
    /// Creates a new `MutexWithTimeout` with the default timeout.
    pub fn new(inner: T) -> Self {
        Self {
            inner: Mutex::new(inner),
            timeout: Duration::from_secs(GLOBAL_STD_TIMEOUT.load(std::sync::atomic::Ordering::Relaxed)),
        }
    }

    /// Creates a new `MutexWithTimeout` with the given timeout.
    pub fn new_with_timeout(inner: T, timeout: Duration) -> Self {
        Self {
            inner: Mutex::new(inner),
            timeout,
        }
    }


    /// Will attempt to lock the inner `std::sync::Mutex`.
    /// If the lock is not acquired within the timeout, `None` will be returned.
    pub fn lock(&self) -> Option<MutexGuard<T>> {
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
    pub fn try_lock(&self) -> Option<std::sync::MutexGuard<T>> {
        self.inner.try_lock().ok()
    }
}