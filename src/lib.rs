pub mod std;
#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(test)]
mod tests {
    use ::std::time::Duration;
    use super::*;

    #[test]
    fn test_lock() {
        let mutex = std::MutexWithTimeout::new((), Duration::from_secs(1));
        let _guard = mutex.lock().unwrap();
    }
}
