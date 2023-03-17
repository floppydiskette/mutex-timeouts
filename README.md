# mutex-timeouts
an incredibly simple crate that allows you to create mutexes with timeouts.  
should be a nearly drop-in replacement for std::sync::Mutex, or tokio::sync::Mutex;
however, you will need to specify a timeout duration when creating the mutex.
```rust
use mutex_timeouts::std::MutexWithTimeout as Mutex;
use std::time::Duration;

let mutex = Mutex::new_with_timeout(0, Duration::from_secs(1));

// this will block for at most 1 second, and then return an error if the mutex is still locked
let _ = mutex.lock().unwrap();

// this will block for at most one second, but not return an error if the mutex is still locked
if let Some(guard) = mutex.try_lock().unwrap() {
    // do something with the guard
}

let mutex = Mutex::new(0); // same as above, but with a default timeout of 5 seconds
```