// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Where each network was last heard, so a join tunes there first instead of
//! sweeping every channel the domain opens.

use crate::regdom::Band;

const SLOTS: usize = 16;

#[derive(Clone, Copy)]
struct Hint {
    ssid: [u8; 32],
    len: u8,
    band: Band,
    channel: u8,
}

pub struct Hints {
    slots: [Option<Hint>; SLOTS],
    next: usize,
}

impl Hints {
    pub const fn new() -> Self {
        Self { slots: [None; SLOTS], next: 0 }
    }

    /// Record `ssid` on a channel; a name already held is moved, a new one
    /// takes the oldest slot.
    pub fn note(&mut self, ssid: &[u8], band: Band, channel: u8) {
        if ssid.is_empty() || ssid.len() > 32 {
            return;
        }
        let at = self.find(ssid).unwrap_or_else(|| {
            let at = self.next;
            self.next = (self.next + 1) % SLOTS;
            at
        });
        let mut name = [0u8; 32];
        name[..ssid.len()].copy_from_slice(ssid);
        self.slots[at] = Some(Hint { ssid: name, len: ssid.len() as u8, band, channel });
    }

    /// The channel `ssid` was last heard on.
    pub fn lookup(&self, ssid: &[u8]) -> Option<(Band, u8)> {
        let h = self.slots[self.find(ssid)?]?;
        Some((h.band, h.channel))
    }

    fn find(&self, ssid: &[u8]) -> Option<usize> {
        self.slots.iter().position(|s| s.is_some_and(|h| &h.ssid[..h.len as usize] == ssid))
    }
}

impl Default for Hints {
    fn default() -> Self {
        Self::new()
    }
}
