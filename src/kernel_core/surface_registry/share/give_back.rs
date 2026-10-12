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

//! A failed attach gives back the reference it took (L46): it used to keep
//! it, so the surface could never be released to zero.

use crate::kernel_core::surface_registry::table::SLOTS;

pub(super) fn give_back(idx: u32, epoch: u32) {
    let mut slots = SLOTS.lock();
    if let Some(slot) = slots.get_mut(idx as usize).and_then(|s| s.as_mut()) {
        if slot.epoch == epoch {
            slot.refcount = slot.refcount.saturating_sub(1);
        }
    }
}
