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

//! What each root port showed when it was last asked, and when it may be
//! asked again. A port whose device failed is asked again at once a few
//! times, then after a wait that doubles; one whose device had no medium
//! waits from the first, so an empty card reader is not reset every few
//! seconds. A port no longer listed is forgotten, so a device plugged into
//! it anew is asked at once.

use crate::clock::Deadline;
use crate::xhci::Port;

/// Failures in a row a port is asked again at once for.
pub(super) const TRIES: u8 = 3;
/// The first wait after them, doubled each time up to 80 seconds.
const FIRST_MS: u64 = 5_000;
const DOUBLINGS: u8 = 4;

#[derive(Clone, Copy)]
pub(super) enum Seen {
    New,
    /// Another class's device, or not the one served: not asked while it stays.
    NotOurs,
    /// Failed, or had no medium, `tries` times in a row.
    Later {
        tries: u8,
        next: Deadline,
    },
}

pub struct Ports(pub(super) [Seen; 256]);

impl Ports {
    pub fn new() -> Self {
        Self([Seen::New; 256])
    }

    /// Forget every port `listed` does not have.
    pub fn forget_gone(&mut self, listed: &[Port]) {
        for (id, seen) in self.0.iter_mut().enumerate() {
            if !listed.iter().any(|p| usize::from(p.id) == id) {
                *seen = Seen::New;
            }
        }
    }

    /// A device of another class, or (`not_ours` false) one to ask afresh.
    pub fn set(&mut self, id: u8, not_ours: bool) {
        self.0[usize::from(id)] = if not_ours { Seen::NotOurs } else { Seen::New };
    }

    /// A device that failed, or had no medium (`empty`), and waits from the
    /// first time.
    pub fn failed(&mut self, id: u8, empty: bool) {
        let seen = &mut self.0[usize::from(id)];
        let before = if let Seen::Later { tries, .. } = *seen { tries } else { 0 };
        let tries = if empty { before.max(TRIES - 1) } else { before }.saturating_add(1);
        let wait = tries.checked_sub(TRIES).map_or(0, |over| FIRST_MS << over.min(DOUBLINGS));
        *seen = Seen::Later { tries, next: Deadline::after_ms(wait) };
    }
}
