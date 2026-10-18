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

//! How the part answers, what it saw, and the handle a test keeps on it.

use std::sync::{Arc, Mutex};

use nonos_devmodel::LiveDevice;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Spec,
    Reject,
    WrongDescriptor,
    /// The published head with bit 16 set: equal to it only if the id is
    /// cut to 16 bits.
    AliasedDescriptor,
    /// The right head, but the used index moved by two: an answer to a
    /// command that was never published.
    SkippedIndex,
    BadEdidMagic,
    WrongFence,
    /// As `Spec`, but every command of this type is left unanswered.
    StallOn(u32),
}

/// One request as the part saw it: the bytes, the flags on each side of the
/// chain, and the device status it was taken under (zero from a part that
/// does not read the status).
pub struct Seen {
    pub request: Vec<u8>,
    pub request_flags: u16,
    pub response_flags: u16,
    pub status: u8,
}

pub struct Part {
    pub seen: Arc<Mutex<Vec<Seen>>>,
    _live: LiveDevice,
}

impl Part {
    pub fn new(seen: Arc<Mutex<Vec<Seen>>>, live: LiveDevice) -> Self {
        Self { seen, _live: live }
    }

    pub fn seen(&self) -> Vec<Vec<u8>> {
        self.taken(|s| s.request.clone())
    }

    /// The device status each command was taken under, oldest first.
    pub fn statuses(&self) -> Vec<u8> {
        self.taken(|s| s.status)
    }

    fn taken<T>(&self, each: impl Fn(&Seen) -> T) -> Vec<T> {
        let seen = self.seen.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        seen.iter().map(each).collect()
    }
}
