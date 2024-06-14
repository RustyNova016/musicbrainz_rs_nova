use std::sync::Arc;
use std::time::Duration;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::time::sleep_until;
use tokio::time::Instant;

static RATE_LIMIT_NEXT_SPOT: Lazy<Arc<Mutex<Instant>>> =
    Lazy::new(|| Arc::new(Mutex::new(Instant::now())));

/// Wait for the next rate limit window, with concurency checks
pub(super) async fn wait_for_ratelimit() {
    let rate_limit_clone = RATE_LIMIT_NEXT_SPOT.clone();

    // We get the next request window in write
    #[allow(clippy::await_holding_lock)]
    // Clippy complains that we hog the lock during the wait. But this is exactly what we want. So we ignore this lint in this case
    let mut next_slot = rate_limit_clone.lock().await;

    // Then we wait for window to come
    let deadline = *next_slot;
    sleep_until(deadline).await;

    // We set the next window to be the next second. According to MB's documentation, the user should limit itself to 1 request / second
    *next_slot = Instant::now() + Duration::from_secs(1);
}
