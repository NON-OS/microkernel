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

//! The frames an exiting group's tables must not free, keyed by those
//! tables (L46). Keyed by the thread that froze the list, it was lost when
//! another thread's release was the one that freed the tables.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use spin::Mutex;

use crate::memory::addr::PhysAddr;

static KEPT: Mutex<BTreeMap<u64, Vec<PhysAddr>>> = Mutex::new(BTreeMap::new());

/// Add `frames` to what the tables `tables` keep; a second exit pass adds
/// nothing new.
pub(super) fn keep(tables: u64, frames: Vec<PhysAddr>) {
    if frames.is_empty() {
        return;
    }
    let mut kept = KEPT.lock();
    let list = kept.entry(tables).or_default();
    list.extend(frames);
    list.sort_unstable_by_key(|f| f.as_u64());
    list.dedup();
}

/// The frames the teardown of `tables` leaves alone, handed over once.
/// Whether the tables `tables` are still to be freed, mapping what they keep.
pub(super) fn pending(tables: u64) -> bool {
    tables != 0 && KEPT.lock().contains_key(&tables)
}

pub fn take_kept(tables: u64) -> Vec<PhysAddr> {
    KEPT.lock().remove(&tables).unwrap_or_default()
}
