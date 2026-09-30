/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The link op (is the radio associated, and with what) and leaving.
//!
//! The link reply is `[associated u8][bssid 6][ssid_len u8][ssid]` after the
//! header. A driver that cannot join answers with a status code instead, which
//! is too short to read as a link and so reads as none.

use super::call::{word, OP_DISCONNECT, OP_LINK, WIFI_HDR};
use super::find::Driver;
use crate::network::SSID_MAX;

const LINK_TIMEOUT_MS: u64 = 500;
const DISCONNECT_TIMEOUT_MS: u64 = 2_000;
const LINK_FIXED: usize = WIFI_HDR + 8;

/// The driver's association as it reports it.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Link {
    pub associated: bool,
    pub bssid: [u8; 6],
    ssid: [u8; SSID_MAX],
    ssid_len: usize,
}

impl Link {
    /// The network the radio is associated with; empty when it is not.
    pub fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len]
    }
}

impl Driver {
    /// Ask the driver whether it is associated. `None` when it did not answer
    /// with a link reply.
    pub fn link(&self) -> Option<Link> {
        let mut resp = [0u8; LINK_FIXED + SSID_MAX];
        let n = self.request(OP_LINK, &[], &mut resp, LINK_TIMEOUT_MS)?;
        if n < LINK_FIXED {
            return None;
        }
        let len = (resp[LINK_FIXED - 1] as usize).min(SSID_MAX).min(n - LINK_FIXED);
        let mut link = Link { associated: resp[WIFI_HDR] != 0, ..Default::default() };
        link.bssid.copy_from_slice(&resp[WIFI_HDR + 1..WIFI_HDR + 7]);
        link.ssid[..len].copy_from_slice(&resp[LINK_FIXED..LINK_FIXED + len]);
        link.ssid_len = len;
        Some(link)
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
