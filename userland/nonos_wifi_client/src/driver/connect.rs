/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The join op: WPA2-Personal (or open) association with a passphrase.
//!
//! The call blocks for the whole join: the driver hunts the network's beacon,
//! authenticates, associates and runs the WPA2 four-way handshake, then
//! installs the keys. A WPA3 (SAE) network cannot be joined: neither driver
//! runs SAE, and the handshake it would need is not the WPA2 one they run.

use super::call::{word, OP_CONNECT, WIFI_HDR};
use super::find::Driver;
use crate::network::SSID_MAX;
use crate::saved::PASS_MAX;
use crate::wipe::wipe;

/// Past the beacon hunt and the handshake with the driver's retransmits.
const CONNECT_TIMEOUT_MS: u64 = 20_000;
/// The code a join reports when the driver did not answer.
pub(super) const NO_REPLY: i32 = -101;

/// A join's result: the driver status code (0 joined, negative a named
/// failure, see `join_text`) and how far the handshake got.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ConnectResult {
    pub code: i32,
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    pub probe: u32,
    pub deauth: u32,
    pub to_us: u32,
    pub state: u8,
}

impl Driver {
    /// Join `ssid` with `pass` (empty for an open network). The body holding
    /// the passphrase is wiped before this returns.
    pub fn connect(&self, ssid: &[u8], pass: &[u8]) -> ConnectResult {
        let (sl, pl) = (ssid.len().min(SSID_MAX), pass.len().min(PASS_MAX));
        let mut body = [0u8; 2 + SSID_MAX + PASS_MAX];
        body[0] = sl as u8;
        body[1..1 + sl].copy_from_slice(&ssid[..sl]);
        body[1 + sl] = pl as u8;
        body[2 + sl..2 + sl + pl].copy_from_slice(&pass[..pl]);
        let mut resp = [0u8; 48];
        let got = self.request(OP_CONNECT, &body[..2 + sl + pl], &mut resp, CONNECT_TIMEOUT_MS);
        wipe(&mut body);
        let n = match got {
            Some(n) if n >= WIFI_HDR + 4 => n,
            _ => return ConnectResult { code: NO_REPLY, ..Default::default() },
        };
        let code = word(&resp, WIFI_HDR) as i32;
        if n < 43 {
            return ConnectResult { code, ..Default::default() };
        }
        ConnectResult {
            code,
            sent: word(&resp, 14),
            recv: word(&resp, 18),
            data: word(&resp, 22),
            eapol: word(&resp, 26),
            probe: word(&resp, 30),
            deauth: word(&resp, 34),
            to_us: word(&resp, 38),
            state: resp[42],
        }
    }
}
