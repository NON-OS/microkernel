/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Keeping a network in the list.

use super::list::{SavedList, LEN_MASK, PASS_MAX, SAVED_HIDDEN, SAVED_WPA3, SLOTS};
use crate::join_wire::JoinFlags;
use crate::network::SSID_MAX;

impl SavedList {
    /// Keep `ssid` with `pass`, replacing an entry of the same name and
    /// keeping its flags. False for a name or passphrase the slot cannot hold.
    pub fn put(&mut self, ssid: &[u8], pass: &[u8]) -> bool {
        self.put_with(ssid, pass, JoinFlags::default())
    }

    /// Keep `ssid` with `pass` and `flags`. The flags only add: a network
    /// once saved as WPA3 (or hidden) stays so when saved again, until it is
    /// forgotten, so saving a new passphrase cannot open it to WPA2.
    pub fn put_with(&mut self, ssid: &[u8], pass: &[u8], flags: JoinFlags) -> bool {
        if ssid.is_empty() || ssid.len() > SSID_MAX || pass.len() > PASS_MAX {
            return false;
        }
        let mut keep = if flags.wpa3_only { SAVED_WPA3 } else { 0 };
        keep |= if flags.hidden { SAVED_HIDDEN } else { 0 };
        if let Some(i) = self.find(ssid) {
            keep |= self.slots[i][0] & !LEN_MASK;
            self.remove(i);
        } else if self.count == SLOTS {
            self.remove(0);
        }
        let s = &mut self.slots[self.count];
        s[0] = ssid.len() as u8 | keep;
        s[1] = pass.len() as u8;
        s[2..2 + ssid.len()].copy_from_slice(ssid);
        s[2 + SSID_MAX..2 + SSID_MAX + pass.len()].copy_from_slice(pass);
        self.count += 1;
        true
    }
}
