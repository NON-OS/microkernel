/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The one path a join's passphrase takes to the driver, and the rejoin
//! net_core's autojoin sends after a loss: flagged, so a driver whose link the
//! person left since then refuses it (code -13), and a Leave that lands while
//! a rejoin pass runs is never undone.

use super::call::{OP_CONNECT, WIFI_HDR};
use super::connect::{NO_REPLY, RESP_LEN};
use super::find::Driver;
use super::services::CANNOT_JOIN;
use crate::join_wire::{
    encode_join, parse_connect_reply, ConnectResult, JoinFlags, FLAG_REJOIN, JOIN_BODY_MAX,
};
use crate::wipe::wipe;

impl Driver {
    /// As `connect_within`, flagged as net_core's rejoin of a lost network.
    pub fn rejoin_within(
        &self,
        ssid: &[u8],
        pass: &[u8],
        flags: JoinFlags,
        timeout_ms: u64,
    ) -> Option<ConnectResult> {
        self.send_join(ssid, pass, flags, FLAG_REJOIN, timeout_ms)
    }

    /// Send a join with `extra` set in its flags octet; the body holding the
    /// passphrase is wiped before this returns.
    pub(super) fn send_join(
        &self,
        ssid: &[u8],
        pass: &[u8],
        flags: JoinFlags,
        extra: u8,
        timeout_ms: u64,
    ) -> Option<ConnectResult> {
        // A driver that cannot join would answer CANNOT_JOIN after the
        // passphrase had crossed to it; answer for it and send nothing.
        if !self.joins() {
            return Some(ConnectResult { code: CANNOT_JOIN, ..Default::default() });
        }
        let mut body = [0u8; JOIN_BODY_MAX];
        let len = encode_join(ssid, pass, flags, &mut body);
        body[len - 1] |= extra;
        let mut resp = [0u8; RESP_LEN];
        let got = self.request(OP_CONNECT, &body[..len], &mut resp, timeout_ms);
        wipe(&mut body);
        let n = got?;
        let reply = resp.get(WIFI_HDR..n).and_then(parse_connect_reply);
        Some(reply.unwrap_or(ConnectResult { code: NO_REPLY, ..Default::default() }))
    }
}
