/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */


//! The join op: WPA3-Personal (SAE), WPA2-Personal or open association with a
//! passphrase.
//!
//! The call blocks for the whole join: the driver hunts the network's beacon,
//! authenticates (SAE when the network offers it), associates and runs the
//! four-way handshake, then installs the keys. Every join carries the flags
//! the saved list holds for the network, whoever asks for it, so a network
//! saved as WPA3 is never joined with WPA2 from any panel.

use super::call::{OP_CONNECT, WIFI_HDR};
use super::find::Driver;
use super::services::CANNOT_JOIN;
use crate::join_wire::{encode_join, parse_connect_reply, ConnectResult, JoinFlags, JOIN_BODY_MAX, JOIN_PASS_MAX};
use crate::saved::{saved_flags, PASS_MAX};
use crate::wipe::wipe;

/// Past the beacon hunt and the handshake with the driver's retransmits.
const CONNECT_TIMEOUT_MS: u64 = 30_000;
/// The code a join reports when the driver did not answer.
pub(super) const NO_REPLY: i32 = -101;
/// The reply: the header, the connect body and room to spare.
const RESP_LEN: usize = WIFI_HDR + 38;

// The wire's passphrase bound is the saved list's.
const _: () = assert!(JOIN_PASS_MAX == PASS_MAX);

impl Driver {
    /// Join `ssid` with `pass` (empty for an open network), under the flags
    /// the saved list holds for it (none for a network not saved). The body
    /// holding the passphrase is wiped before this returns.
    pub fn connect(&self, ssid: &[u8], pass: &[u8]) -> ConnectResult {
        self.connect_with(ssid, pass, saved_flags(ssid))
    }

    /// Join `ssid` with `pass` under `flags`, for a caller that already holds
    /// the saved list (net_core's autojoin).
    pub fn connect_with(&self, ssid: &[u8], pass: &[u8], flags: JoinFlags) -> ConnectResult {
        self.connect_within(ssid, pass, flags, CONNECT_TIMEOUT_MS)
            .unwrap_or(ConnectResult { code: NO_REPLY, ..Default::default() })
    }

    /// Ask the driver to join and wait at most `timeout_ms` for how it went.
    /// `None` when no reply came in that time: the driver has the request and
    /// the join runs on, so the caller learns how it ended from `link`. For
    /// net_core, whose loop serves every other client and cannot wait out a
    /// join; its late reply is dropped by the kernel's call correlation.
    pub fn connect_within(
        &self,
        ssid: &[u8],
        pass: &[u8],
        flags: JoinFlags,
        timeout_ms: u64,
    ) -> Option<ConnectResult> {
        // A driver that cannot join would answer CANNOT_JOIN after the
        // passphrase had crossed to it; answer for it and send nothing.
        if !self.joins() {
            return Some(ConnectResult { code: CANNOT_JOIN, ..Default::default() });
        }
        let mut body = [0u8; JOIN_BODY_MAX];
        let len = encode_join(ssid, pass, flags, &mut body);
        let mut resp = [0u8; RESP_LEN];
        let got = self.request(OP_CONNECT, &body[..len], &mut resp, timeout_ms);
        wipe(&mut body);
        let n = got?;
        let reply = resp.get(WIFI_HDR..n).and_then(parse_connect_reply);
        Some(reply.unwrap_or(ConnectResult { code: NO_REPLY, ..Default::default() }))
    }
}
