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

/*
 * Messages taken out of the SynIC slot and not yet read by the VMBus core.
 * Taking them on VP 0 is what frees the slot and lets the host send the
 * next; the core reads them from here on whichever CPU it runs.
 */

use super::message::MESSAGE_BYTES;

pub(super) const RING_SLOTS: usize = 32;

pub(super) struct Ring {
    slots: [[u8; MESSAGE_BYTES]; RING_SLOTS],
    head: usize,
    len: usize,
}

impl Ring {
    pub(super) const fn new() -> Self {
        Self { slots: [[0; MESSAGE_BYTES]; RING_SLOTS], head: 0, len: 0 }
    }

    /// False when full: the message stays in the slot, and the host waits.
    pub(super) fn push(&mut self, m: &[u8; MESSAGE_BYTES]) -> bool {
        if self.len == RING_SLOTS {
            return false;
        }
        self.slots[(self.head + self.len) % RING_SLOTS] = *m;
        self.len += 1;
        true
    }

    pub(super) fn pop(&mut self) -> Option<[u8; MESSAGE_BYTES]> {
        if self.len == 0 {
            return None;
        }
        let m = self.slots[self.head];
        self.head = (self.head + 1) % RING_SLOTS;
        self.len -= 1;
        Some(m)
    }
}

impl Default for Ring {
    fn default() -> Self {
        Self::new()
    }
}
