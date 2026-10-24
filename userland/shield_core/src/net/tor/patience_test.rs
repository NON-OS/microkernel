// NONOS Operating System (AGPL-3.0-or-later)
//! The patience a stream keeps and the milliseconds it gives nonos_tls.
//!
//! The route values (Nym 60 000, Anyone 30 000) are nonos_route_link's `Route::patience_ms`,
//! a dependency on the nonos target only, so here they enter as the numbers that function
//! answers; stream_open passes the function's own answer.

use super::{patience_as_ms, patience_for, FLOOR};
use std::time::Duration;

const NYM_MS: u64 = 60_000;
const ANON_MS: u64 = 30_000;
const CALL: Duration = Duration::from_secs(300);

#[test]
fn milliseconds_are_exact_and_saturate() {
    assert_eq!(patience_as_ms(Duration::from_secs(60)), 60_000);
    assert_eq!(patience_as_ms(FLOOR), 1_000);
    assert_eq!(patience_as_ms(Duration::MAX), i64::MAX);
    assert_eq!(patience_as_ms(Duration::ZERO), 0);
}

#[test]
fn a_stream_keeps_its_routes_patience() {
    assert_eq!(patience_as_ms(patience_for(NYM_MS, CALL)), 60_000);
    assert_eq!(patience_as_ms(patience_for(ANON_MS, CALL)), 30_000);
}

#[test]
fn the_call_limit_caps_it_and_a_second_floors_it() {
    assert_eq!(patience_for(NYM_MS, Duration::from_secs(10)), Duration::from_secs(10));
    assert_eq!(patience_for(0, CALL), FLOOR);
    assert_eq!(patience_for(NYM_MS, Duration::ZERO), FLOOR);
    assert_eq!(patience_for(u64::MAX, CALL), CALL);
}
