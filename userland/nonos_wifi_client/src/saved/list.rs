/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The saved networks in plaintext, only ever held in memory.
//!
//! Each slot is `[ssid_len][pass_len][ssid 32][pass 64]`, so the list encodes
//! to a fixed length: the store replaces a record only with one of the same
//! length. The bytes are wiped when the list is dropped.

use crate::network::SSID_MAX;

/// The most networks kept. Remembering one more drops the oldest.
pub const SLOTS: usize = 4;
/// A WPA2 passphrase is 8 to 63 characters, or 64 hex digits of key.
pub const PASS_MAX: usize = 64;
pub(super) const SLOT_LEN: usize = 2 + SSID_MAX + PASS_MAX;
pub(super) const PLAIN_LEN: usize = SLOTS * SLOT_LEN;

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
        &s[2..2 + (s[0] as usize).min(SSID_MAX)]
    }

    pub fn passphrase(&self, i: usize) -> &[u8] {
        let s = &self.slots[i.min(SLOTS - 1)];
        &s[2 + SSID_MAX..2 + SSID_MAX + (s[1] as usize).min(PASS_MAX)]
    }

    pub fn find(&self, ssid: &[u8]) -> Option<usize> {
        (0..self.count).find(|&i| self.ssid(i) == ssid)
    }

    /// Keep `ssid` with `pass`, replacing an entry of the same name. False for
    /// a name or passphrase the slot cannot hold.
    pub fn put(&mut self, ssid: &[u8], pass: &[u8]) -> bool {
        if ssid.is_empty() || ssid.len() > SSID_MAX || pass.len() > PASS_MAX {
            return false;
        }
        if let Some(i) = self.find(ssid) {
            self.remove(i);
        } else if self.count == SLOTS {
            self.remove(0);
        }
        let s = &mut self.slots[self.count];
        s[0] = ssid.len() as u8;
        s[1] = pass.len() as u8;
        s[2..2 + ssid.len()].copy_from_slice(ssid);
        s[2 + SSID_MAX..2 + SSID_MAX + pass.len()].copy_from_slice(pass);
        self.count += 1;
        true
    }
}
