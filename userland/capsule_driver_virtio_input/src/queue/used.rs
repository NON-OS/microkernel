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

//! Reading the used ring, one element per buffer the device returned, and
//! the buffer itself.

use core::ptr::read_volatile;
use core::sync::atomic::{fence, Ordering};

use super::ring::EventRing;
use crate::constants::queue::{EVENTS_OFF, EVENT_BYTES, USED_OFF};

/// A used element: which descriptor, and how many bytes the device wrote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Used {
    pub id: u32,
    pub len: u32,
}

// SAFETY anchor: see ring.rs.
impl EventRing {
    pub fn take(&mut self) -> Option<Used> {
        let idx = unsafe { read_volatile(self.at::<u16>(USED_OFF + 2)) };
        if idx == self.last_used {
            return None;
        }
        // The element and the buffer are the driver's only once the index
        // that covers them is seen.
        fence(Ordering::Acquire);
        let slot = (self.last_used % self.size) as usize;
        let e = USED_OFF + 4 + slot * 8;
        let used =
            unsafe { Used { id: read_volatile(self.at(e)), len: read_volatile(self.at(e + 4)) } };
        self.last_used = self.last_used.wrapping_add(1);
        Some(used)
    }

    /// The eight bytes the device wrote into buffer `id`.
    pub fn event(&self, id: u16) -> [u8; EVENT_BYTES] {
        let off = EVENTS_OFF + (id % self.size) as usize * EVENT_BYTES;
        unsafe { read_volatile(self.at::<[u8; EVENT_BYTES]>(off)) }
    }
}
