/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Keeping a network in the list.

use super::dropped::Dropped;
use super::list::{SavedList, LEN_MASK, SAVED_HIDDEN, SAVED_WPA3, SLOTS};
use super::pass::usable;
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
        self.put_dropping(ssid, pass, flags).is_some()
    }

    /// As `put_with`, saying which network was dropped to make room: the
    /// oldest one not pinned to WPA3, or the oldest of all when all are, so
    /// a pin is lost only when nothing else can go. None when refused.
    pub fn put_dropping(
        &mut self,
        ssid: &[u8],
        pass: &[u8],
        flags: JoinFlags,
    ) -> Option<Option<Dropped>> {
        if ssid.is_empty() || ssid.len() > SSID_MAX {
            return None;
        }
        let mut dropped = None;
        let mut keep = if flags.wpa3_only { SAVED_WPA3 } else { 0 };
        keep |= if flags.hidden { SAVED_HIDDEN } else { 0 };
        let found = self.find(ssid);
        if let Some(i) = found {
            keep |= self.slots[i][0] & !LEN_MASK;
        }
        if !usable(pass, keep & SAVED_WPA3 != 0) {
            return None;
        }
        if let Some(i) = found {
            self.remove(i);
        } else if self.count == SLOTS {
            let pinned = |i: usize| self.slots[i][0] & SAVED_WPA3 != 0;
            let i = (0..SLOTS).find(|&i| !pinned(i)).unwrap_or(0);
            dropped = Some(Dropped::of(self.ssid(i)));
            self.remove(i);
        }
        let s = &mut self.slots[self.count];
        s[0] = ssid.len() as u8 | keep;
        s[1] = pass.len() as u8;
        s[2..2 + ssid.len()].copy_from_slice(ssid);
        s[2 + SSID_MAX..2 + SSID_MAX + pass.len()].copy_from_slice(pass);
        self.count += 1;
        Some(dropped)
    }
}
