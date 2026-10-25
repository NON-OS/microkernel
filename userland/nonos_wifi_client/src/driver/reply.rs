/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The request header, and the reply a request takes: its own header back,
//! whole and inside the buffer. Anything else is not the driver's answer.

/// The family's tag.
pub const WIFI_MAGIC: u32 = 0x5749_4649;
/// The header both directions carry: magic, op and request id.
pub const WIFI_HDR: usize = 10;

/// The header of request `rid` for `op`.
pub fn header(op: u16, rid: u32) -> [u8; WIFI_HDR] {
    let mut h = [0u8; WIFI_HDR];
    h[0..4].copy_from_slice(&WIFI_MAGIC.to_le_bytes());
    h[4..6].copy_from_slice(&op.to_le_bytes());
    h[6..10].copy_from_slice(&rid.to_le_bytes());
    h
}

/// The reply's length when the call's result `n` says `resp` holds the
/// answer to request `rid` for `op`: at least a header, no more than the
/// buffer, and the request's magic, op and id echoed.
pub fn answer(resp: &[u8], n: i64, op: u16, rid: u32) -> Option<usize> {
    let n = usize::try_from(n).ok().filter(|&n| n >= WIFI_HDR && n <= resp.len())?;
    (resp[..WIFI_HDR] == header(op, rid)).then_some(n)
}
