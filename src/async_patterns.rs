//! Async/await patterns
//!
//! Asynchronous programming in Rust requires careful handling of futures
//! and async traits. These examples show common patterns.

use std::future::Future;
use std::pin::Pin;

/// Basic async function
pub async fn simple_async() -> i32 {
    42
}

/// Async function that awaits another async function
pub async fn composed_async() -> i32 {
    let result = simple_async().await;
    result + 1
}

/// Async function with error handling
pub async fn async_with_result() -> Result<String, String> {
    let value = simple_async().await;
    if value > 0 {
        Ok(format!("Value: {}", value))
    } else {
        Err("Invalid value".to_string())
    }
}

/// Multiple concurrent operations (conceptual - would need tokio::join! in real code)
pub async fn concurrent_operations() -> (i32, i32) {
    let a = simple_async().await;
    let b = simple_async().await;
    (a, b)
}

/// Async trait method (requires manual implementation pre-async-trait)
pub trait AsyncProcessor {
    fn process(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}

pub struct SimpleProcessor {
    pub name: String,
}

impl AsyncProcessor for SimpleProcessor {
    fn process(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        let name = self.name.clone();
        Box::pin(async move { format!("Processing: {}", name) })
    }
}

/// Async function returning a boxed future
pub fn boxed_future() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async { 42 })
}

/// Async closure (using async block)
pub fn async_closure_example() -> impl Future<Output = i32> {
    async {
        let value = simple_async().await;
        value * 2
    }
}

/// Generic async function
pub async fn generic_async<T>(value: T) -> T
where
    T: Send + 'static,
{
    value
}

/// Async function with lifetime parameters
pub async fn async_with_lifetime<'a>(s: &'a str) -> &'a str {
    s
}

/// Stream-like pattern (iterator equivalent for async)
pub struct AsyncCounter {
    count: u32,
    max: u32,
}

impl AsyncCounter {
    pub fn new(max: u32) -> Self {
        AsyncCounter { count: 0, max }
    }

    pub async fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// Conditional async execution
pub async fn maybe_async(do_work: bool) -> String {
    if do_work {
        simple_async().await;
        "Work done".to_string()
    } else {
        "No work".to_string()
    }
}

/// Async function that takes a future as parameter
pub async fn execute_future<F>(future: F) -> i32
where
    F: Future<Output = i32>,
{
    future.await
}

#[cfg(test)]
mod tests {
    // Note: These tests would need an async runtime like tokio to actually run
    // They're shown here to demonstrate the pattern

    #[test]
    fn test_async_counter() {
        // In real code, you'd use #[tokio::test] or similar
        // let mut counter = AsyncCounter::new(3);
        // This demonstrates the pattern even if we can't run it
    }
}
