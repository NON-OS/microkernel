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

//! Which broker handle each open description holds.
//!
//! Keyed by the description's number (desc::of), which dup and fork copy:
//! descriptors sharing a description share the broker handle, and the
//! last close of the description lets it go (locks/lock_calls/closing.rs).

use alloc::collections::BTreeMap;
use core::cell::RefCell;

struct Table(RefCell<BTreeMap<u32, u64>>);

// SAFETY: only the serve loop opens and closes; a guest only traps into it.
unsafe impl Sync for Table {}

static HELD: Table = Table(RefCell::new(BTreeMap::new()));

pub(super) fn hold(desc: u32, handle: u64) {
    HELD.0.borrow_mut().insert(desc, handle);
}

/// The broker handle of description `desc`, if it is the broker's.
pub(super) fn handle(desc: u32) -> Option<u64> {
    HELD.0.borrow().get(&desc).copied()
}

/// The description's last descriptor closed: the broker's handle goes.
/// A file written through it is committed within the broker's grace.
pub fn gone(desc: u32) {
    let Some(handle) = HELD.0.borrow_mut().remove(&desc) else {
        return;
    };
    // A close the broker cannot take is reaped when this process ends.
    let _ = super::link::with(|c| c.close(handle));
}
