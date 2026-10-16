/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The saved networks in plaintext, only ever held in memory.
//!
//! Each slot is `[ssid_len][pass_len][ssid 32][pass 64]`, so the list encodes
//! to a fixed length: the store replaces a record only with one of the same
//! length. The SSID length needs six bits; the top two of its octet hold the
//! network's flags: saved as WPA3 (bit 7), so it is never joined with WPA2,
//! and hidden (bit 6), so a probe request may name it. A record written before
//! the flags reads as neither. The bytes are wiped when the list is dropped.

use crate::join_wire::JoinFlags;
use crate::network::SSID_MAX;

/// The most networks kept. Remembering one more drops the oldest.
pub const SLOTS: usize = 4;
/// A WPA2 passphrase is 8 to 63 characters, or 64 hex digits of key.
pub const PASS_MAX: usize = 64;
pub(super) const SLOT_LEN: usize = 2 + SSID_MAX + PASS_MAX;
pub(super) const PLAIN_LEN: usize = SLOTS * SLOT_LEN;
/// The SSID length bits of a slot's first octet.
pub(super) const LEN_MASK: u8 = 0x3F;
pub(super) const SAVED_WPA3: u8 = 0x80;
pub(super) const SAVED_HIDDEN: u8 = 0x40;

pub struct SavedList {
    pub(super) slots: [[u8; SLOT_LEN]; SLOTS],
    pub(super) count: usize,
}

impl SavedList {
    pub const fn new() -> Self {
        Self { slots: [[0; SLOT_LEN]; SLOTS], count: 0 }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn ssid(&self, i: usize) -> &[u8] {
        let s = &self.slots[i.min(SLOTS - 1)];
        &s[2..2 + ((s[0] & LEN_MASK) as usize).min(SSID_MAX)]
    }

    /// How entry `i` must be joined.
    pub fn join_flags(&self, i: usize) -> JoinFlags {
        let s = self.slots[i.min(SLOTS - 1)][0];
        JoinFlags { wpa3_only: s & SAVED_WPA3 != 0, hidden: s & SAVED_HIDDEN != 0 }
    }

    pub fn passphrase(&self, i: usize) -> &[u8] {
        let s = &self.slots[i.min(SLOTS - 1)];
        &s[2 + SSID_MAX..2 + SSID_MAX + (s[1] as usize).min(PASS_MAX)]
    }

    pub fn find(&self, ssid: &[u8]) -> Option<usize> {
        (0..self.count).find(|&i| self.ssid(i) == ssid)
    }
}
