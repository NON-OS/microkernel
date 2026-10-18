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

//! Which pid holds which ACPI window: a second capsule is refused a window
//! the first holds, and a window asked for again is not logged again.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::alive::alive;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) enum Space {
    Memory,
    Io,
    Pci,
}

#[derive(Clone, Copy)]
struct Held {
    pid: u32,
    space: Space,
    base: u64,
    end: u64,
}

/// Far past any firmware's region count; a capsule asking for more is
/// refused rather than left to grow the kernel heap.
const MAX_HELD: usize = 1024;

static HELD: Mutex<Vec<Held>> = Mutex::new(Vec::new());

/// Whether `pid` already holds all of [base, end).
pub(in super::super) fn holds(pid: u32, space: Space, base: u64, end: u64) -> bool {
    let held = HELD.lock();
    held.iter().any(|h| h.pid == pid && h.space == space && h.base <= base && end <= h.end)
}

/// Whether a live capsule other than `pid` holds any of [base, end).
pub(in super::super) fn other_holds(pid: u32, space: Space, base: u64, end: u64) -> bool {
    let held = HELD.lock();
    held.iter()
        .any(|h| h.pid != pid && h.space == space && h.base < end && base < h.end && alive(h.pid))
}

/// Record a grant; false when the table is full even of the living.
pub(in super::super) fn insert(pid: u32, space: Space, base: u64, end: u64) -> bool {
    let mut held = HELD.lock();
    if held.len() >= MAX_HELD {
        // Grants are not released one by one; a capsule's go when it ends.
        held.retain(|h| alive(h.pid));
    }
    if held.len() >= MAX_HELD {
        return false;
    }
    held.push(Held { pid, space, base, end });
    true
}
