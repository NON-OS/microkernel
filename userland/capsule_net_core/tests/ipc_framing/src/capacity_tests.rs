//! How much a read may drain, from what the caller says it can hold.

use crate::server::parse_req::IPC_BUF_MAX;
use crate::server::recv_cap::recv_cap;

/// A caller that states its capacity is held to it, because a read consumes
/// what it copies and the remainder cannot be asked for again.
#[test]
fn a_stated_capacity_bounds_the_drain() {
    let mut body = [0u8; 8];
    body[4..8].copy_from_slice(&512u32.to_le_bytes());
    assert_eq!(recv_cap(&body), 512);
}

/// An oversized claim cannot talk the server into a reply the caller could
/// not hold either.
#[test]
fn a_stated_capacity_cannot_exceed_the_inbox() {
    let mut body = [0u8; 8];
    body[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(recv_cap(&body), IPC_BUF_MAX);
}

/// A caller from before the field keeps the size it was built against, so
/// widening the inbox does not start overrunning it.
#[test]
fn a_caller_without_the_field_keeps_the_old_size() {
    assert_eq!(recv_cap(&[0u8; 4]), 1024);
}

/// Zero would allocate nothing and report an empty socket forever.
#[test]
fn a_zero_capacity_still_leaves_room_to_read() {
    let body = [0u8; 8];
    assert!(recv_cap(&body) >= 1);
}
