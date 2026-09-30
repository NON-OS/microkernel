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

//! The slots held by live runs: freed when a run ends, found by the
//! terminal that started them, and asked after by the debug channel.

use alloc::vec::Vec;

use super::slots::{Slot, SLOTS};

/// Free the slot `pid` holds, once it has exited and its endpoints are
/// gone, so the next run can register them again.
pub(super) fn release(pid: u32) {
    let mut slots = SLOTS.lock();
    for s in slots.iter_mut() {
        if matches!(*s, Slot::Held { pid: p, .. } if p == pid) {
            *s = Slot::Free;
        }
    }
}

/// The live runs `parent` started.
pub(super) fn held_by(parent: u32) -> Vec<u32> {
    let slots = SLOTS.lock();
    slots
        .iter()
        .filter_map(|s| match *s {
            Slot::Held { parent: p, pid } if p == parent => Some(pid),
            _ => None,
        })
        .collect()
}

/// Whether `pid` holds a slot.
pub(super) fn holds(pid: u32) -> bool {
    SLOTS.lock().iter().any(|s| matches!(*s, Slot::Held { pid: p, .. } if p == pid))
}
