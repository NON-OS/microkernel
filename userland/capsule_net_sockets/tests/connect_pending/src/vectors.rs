//! A connect is answered when its handshake resolves or its time runs out,
//! and at no other moment.

use crate::verdict::{decide, Verdict};

const SYN_SENT: u8 = 1;
const SYN_RECEIVED: u8 = 2;
const ESTABLISHED: u8 = 3;
const CLOSE_WAIT: u8 = 4;
const CLOSED: u8 = 0xFF;

#[test]
fn a_completed_handshake_is_answered_as_connected() {
    assert_eq!(decide(Ok(ESTABLISHED), 100, 8000), Verdict::Up);
}

/// A server that closes right after the handshake still completed it.
#[test]
fn a_peer_that_closed_at_once_still_connected() {
    assert_eq!(decide(Ok(CLOSE_WAIT), 100, 8000), Verdict::Up);
}

#[test]
fn a_refusal_or_a_lost_connection_is_answered_as_failed() {
    assert_eq!(decide(Ok(CLOSED), 100, 8000), Verdict::Failed);
    assert_eq!(decide(Err(7), 100, 8000), Verdict::Failed);
}

/// The regression the table exists for: a handshake in progress keeps its
/// caller waiting without holding the service.
#[test]
fn a_handshake_in_progress_waits_until_the_deadline() {
    assert_eq!(decide(Ok(SYN_SENT), 7999, 8000), Verdict::Waiting);
    assert_eq!(decide(Ok(SYN_RECEIVED), 8000, 8000), Verdict::Waiting);
    assert_eq!(decide(Ok(SYN_SENT), 8001, 8000), Verdict::Failed);
}

/// The millisecond clock is compared by difference, so a deadline set just
/// before it wraps still expires after it.
#[test]
fn the_deadline_survives_the_clock_wrapping() {
    let deadline = i64::MAX.wrapping_add(10);
    assert_eq!(decide(Ok(SYN_SENT), i64::MAX, deadline), Verdict::Waiting);
    assert_eq!(decide(Ok(SYN_SENT), deadline.wrapping_add(1), deadline), Verdict::Failed);
}
