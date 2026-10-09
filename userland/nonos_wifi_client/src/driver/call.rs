/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! One request in the Wi-Fi control family and the wait for its reply.
//!
//! Both drivers answer this family on their service inbox, told apart from
//! net_core's link protocol (and from the iwlwifi debug protocol) by its tag:
//! `[magic u32][op u16][request id u32]` then the body. The reply echoes the
//! header and carries the op's fields after it.

use nonos_libc::mk_ipc_call_timeout;

use crate::join_wire::JOIN_BODY_MAX;
use crate::wipe::wipe;

const WIFI_MAGIC: u32 = 0x5749_4649;
/// The header both directions carry: magic, op and request id.
pub const WIFI_HDR: usize = 10;
pub(super) const OP_CONNECT: u16 = 1;
pub(super) const OP_DISCONNECT: u16 = 2;
pub(super) const OP_SCAN: u16 = 3;
/// How far bring-up got; the RTL8821CE also appends its data-path counters.
pub const OP_STATUS: u16 = 4;
pub(super) const OP_LINK: u16 = 5;
/// The largest body sent: a join's `[ssid_len][ssid][pass_len][pass][flags]`.
const BODY_MAX: usize = JOIN_BODY_MAX;

/// Send `op` with `body` to `port` and wait up to `timeout_ms`. Returns the
/// reply length when at least a header came back. The request buffer is wiped
/// before returning, since a join's body holds the passphrase.
pub(super) fn call(
    port: u32,
    op: u16,
    body: &[u8],
    resp: &mut [u8],
    timeout_ms: u64,
) -> Option<usize> {
    if body.len() > BODY_MAX {
        return None;
    }
    let mut req = [0u8; WIFI_HDR + BODY_MAX];
    req[0..4].copy_from_slice(&WIFI_MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&op.to_le_bytes());
    req[WIFI_HDR..WIFI_HDR + body.len()].copy_from_slice(body);
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        WIFI_HDR + body.len(),
        resp.as_mut_ptr(),
        resp.len(),
        timeout_ms,
    );
    wipe(&mut req);
    if n < WIFI_HDR as i64 {
        return None;
    }
    Some(n as usize)
}

/// A little-endian u32 at `at`, or zero past the end.
pub(super) fn word(b: &[u8], at: usize) -> u32 {
    match b.get(at..at + 4) {
        Some(w) => u32::from_le_bytes([w[0], w[1], w[2], w[3]]),
        None => 0,
    }
}
