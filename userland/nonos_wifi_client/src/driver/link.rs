/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */


//! The link op (is the radio associated, and with what) and leaving.
//!
//! The link reply is `[associated u8][bssid 6][ssid_len u8][ssid][akm u8]`
//! after the header (`join_wire::parse_link`). A driver that cannot join
//! answers with a status code instead, which is too short to read as a link
//! and so reads as none.

use super::call::{word, OP_DISCONNECT, OP_LINK, WIFI_HDR};
use super::find::Driver;
use crate::join_wire::{parse_link, Link};
use crate::network::SSID_MAX;

const LINK_TIMEOUT_MS: u64 = 500;
const DISCONNECT_TIMEOUT_MS: u64 = 2_000;
/// The header, the fixed link fields, the SSID and the AKM.
const RESP_LEN: usize = WIFI_HDR + 8 + SSID_MAX + 1;

impl Driver {
    /// Ask the driver whether it is associated. `None` when it did not answer
    /// with a link reply.
    pub fn link(&self) -> Option<Link> {
        self.link_within(LINK_TIMEOUT_MS)
    }

    /// `link`, waiting at most `timeout_ms`: `None` also when a driver busy
    /// joining did not answer in time.
    pub fn link_within(&self, timeout_ms: u64) -> Option<Link> {
        let mut resp = [0u8; RESP_LEN];
        let n = self.request(OP_LINK, &[], &mut resp, timeout_ms)?;
        parse_link(resp.get(WIFI_HDR..n)?)
    }

    /// Leave the network and clear the keys. True when the driver said so.
    pub fn disconnect(&self) -> bool {
        let mut resp = [0u8; WIFI_HDR + 4];
        match self.request(OP_DISCONNECT, &[], &mut resp, DISCONNECT_TIMEOUT_MS) {
            Some(n) if n >= WIFI_HDR + 4 => word(&resp, WIFI_HDR) == 0,
            _ => false,
        }
    }
}
