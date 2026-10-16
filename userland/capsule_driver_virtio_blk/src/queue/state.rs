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

//! Whether the queue is free, and where a count falls in the ring.

use super::fault::Fault;
use super::layout::Queue;
use super::ring::{avail_offset, used_offset};

impl Queue {
    /// The queue as the legacy transport drives it (`completion`).
    pub fn on_legacy(mut self) -> Self {
        self.legacy = true;
        self
    }

    /// Why the queue may not be used again before a reset, if it may not.
    pub fn stalled(&self) -> Option<Fault> {
        self.stalled
    }

    /// Give the queue up until a reset. The first reason given is kept.
    pub fn stall(&mut self, why: Fault) -> Fault {
        self.stalled.get_or_insert(why);
        why
    }

    /// The device answered a request OK (`io::submit`).
    pub fn note_served(&mut self) {
        self.served = true;
    }

    /// Whether the device answered OK since this was last asked.
    pub fn take_served(&mut self) -> bool {
        core::mem::take(&mut self.served)
    }

    /// Whether a request is still the device's.
    pub fn outstanding(&self) -> bool {
        self.posted != self.used
    }

    /// Free for a new request, and its buffers for the driver to touch.
    pub(super) fn idle(&self) -> bool {
        self.stalled.is_none() && !self.outstanding()
    }

    pub(super) fn avail(&self) -> usize {
        avail_offset(self.size)
    }

    pub(super) fn used_ring(&self) -> usize {
        used_offset(self.size)
    }

    /// A count reduced to a ring position. A zero size, which bring-up
    /// never hands over, is taken as one entry rather than divided by.
    pub(super) fn pos(&self, idx: u16) -> usize {
        usize::from(idx % self.size.max(1))
    }
}
