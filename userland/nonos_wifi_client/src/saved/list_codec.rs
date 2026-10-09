/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Removing an entry, the plaintext encoding, and wiping on drop.

use super::list::{SavedList, LEN_MASK, PLAIN_LEN, SLOTS, SLOT_LEN};
use crate::network::SSID_MAX;
use crate::wipe::wipe;

impl SavedList {
    /// Drop entry `i`, keeping the rest in order.
    pub fn remove(&mut self, i: usize) {
        if i >= self.count {
            return;
        }
        for j in i..self.count - 1 {
            self.slots[j] = self.slots[j + 1];
        }
        self.count -= 1;
        wipe(&mut self.slots[self.count]);
    }

    /// The slots in order, used ones first; unused slots are zero.
    pub(super) fn encode(&self, out: &mut [u8; PLAIN_LEN]) {
        for (i, slot) in self.slots.iter().enumerate() {
            out[i * SLOT_LEN..(i + 1) * SLOT_LEN].copy_from_slice(slot);
        }
    }

    /// Read slots back until the first empty one. `None` for lengths no slot
    /// could have been written with (flags on an empty name among them), so
    /// a damaged record is not half-read.
    pub(super) fn decode(raw: &[u8]) -> Option<Self> {
        if raw.len() != PLAIN_LEN {
            return None;
        }
        let mut list = SavedList::new();
        for i in 0..SLOTS {
            let slot = &raw[i * SLOT_LEN..(i + 1) * SLOT_LEN];
            if slot[0] == 0 {
                break;
            }
            let ssid_len = (slot[0] & LEN_MASK) as usize;
            if ssid_len == 0 || ssid_len > SSID_MAX || slot[1] as usize > SLOT_LEN - 2 - SSID_MAX {
                return None;
            }
            list.slots[i].copy_from_slice(slot);
            list.count += 1;
        }
        Some(list)
    }
}

impl Default for SavedList {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SavedList {
    fn drop(&mut self) {
        for slot in self.slots.iter_mut() {
            wipe(slot);
        }
    }
}
