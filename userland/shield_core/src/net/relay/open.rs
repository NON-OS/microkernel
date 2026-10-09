//! Reaching a lander's onion service within its share of time: tried again until the time is
//! spent, each connection held to the time left. A connection that fails has carried nothing.

use crate::error::NetError;
use crate::net::tor::{Purpose, Tor, TorStream};
use std::time::{Duration, Instant};

/// The time one lander is given before the next is tried.
pub const BUDGET: Duration = Duration::from_secs(20);
const WAIT: Duration = Duration::from_secs(2);
const WAITS: [u64; 3] = [2, 5, 10];

/// A stream to `onion`, tried again after each failure until its budget is spent.
pub(super) fn open(tor: &Tor, onion: &str) -> Result<TorStream, NetError> {
    let deadline = Instant::now().checked_add(BUDGET).ok_or(NetError::Transport)?;
    let mut last = NetError::Transport;
    loop {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(last);
        }
        match tor.connect_within(Purpose::Relay, (onion, 80), left) {
            Ok(stream) => return Ok(stream),
            Err(e) => last = e,
        }
        std::thread::sleep(WAIT.min(deadline.saturating_duration_since(Instant::now())));
    }
}

/// A read that changes nothing, run again when the answer is lost on the way. A hand-off is never
/// passed here, since a proof sent twice could be queued twice.
pub(super) fn read<T>(mut ask: impl FnMut() -> Result<T, NetError>) -> Result<T, NetError> {
    let mut last = ask();
    for wait in WAITS {
        if !matches!(last, Err(NetError::Transport)) {
            break;
        }
        std::thread::sleep(Duration::from_secs(wait));
        last = ask();
    }
    last
}
