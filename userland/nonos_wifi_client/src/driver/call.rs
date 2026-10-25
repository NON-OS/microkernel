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

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call_timeout;

pub(super) use super::reply::WIFI_HDR;
use super::reply::{answer, header};
use crate::join_wire::JOIN_BODY_MAX;
use crate::wipe::wipe;

/// The next request's id, so a reply is matched to the request it answers.
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
pub(super) const OP_CONNECT: u16 = 1;
pub(super) const OP_DISCONNECT: u16 = 2;
pub(super) const OP_SCAN: u16 = 3;
/// How far bring-up got; the RTL8821CE also appends its data-path counters.
pub const OP_STATUS: u16 = 4;
pub(super) const OP_LINK: u16 = 5;
/// The largest body sent: a join's `[ssid_len][ssid][pass_len][pass][flags]`.
const BODY_MAX: usize = JOIN_BODY_MAX;

/// Send `op` with `body` to `port` and wait up to `timeout_ms`. Returns the
/// reply length when the reply is this request's (`reply::answer`). The
/// request buffer is wiped before returning: a join's body holds the
/// passphrase.
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
    let rid = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let mut req = [0u8; WIFI_HDR + BODY_MAX];
    req[..WIFI_HDR].copy_from_slice(&header(op, rid));
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
    answer(resp, n, op, rid)
}

/// A little-endian u32 at `at`, or zero past the end.
pub(super) fn word(b: &[u8], at: usize) -> u32 {
    match b.get(at..at + 4) {
        Some(w) => u32::from_le_bytes([w[0], w[1], w[2], w[3]]),
        None => 0,
    }
}
