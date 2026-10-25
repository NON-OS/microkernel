// NONOS Operating System (AGPL-3.0-or-later)
//! A stream's patience: its route's wait for a first byte, held to the call's limit and to at
//! least a second, and that wait in the milliseconds nonos_tls asks for. Pure, so the host tests
//! hold both.

use std::time::Duration;

/// The least a stream waits for a first byte.
pub(super) const FLOOR: Duration = Duration::from_secs(1);

/// What a stream over a route that waits `route_ms` keeps, for a call allowed `limit`.
pub(super) fn patience_for(route_ms: u64, limit: Duration) -> Duration {
    Duration::from_millis(route_ms).min(limit).max(FLOOR)
}

/// A patience in milliseconds, saturating rather than wrapping.
pub(super) fn patience_as_ms(d: Duration) -> i64 {
    i64::try_from(d.as_millis()).unwrap_or(i64::MAX)
}

#[cfg(test)]
#[path = "patience_test.rs"]
mod tests;
