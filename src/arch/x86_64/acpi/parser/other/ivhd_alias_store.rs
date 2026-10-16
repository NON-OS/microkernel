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

//! The IVHD aliases, kept from parsing for the AMD-Vi interrupt path (L47).

use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;

use super::ivhd_alias::{aliased, aliases, Alias, MAX_ALIASES};

static ALIASES: Mutex<heapless::Vec<Alias, MAX_ALIASES>> = Mutex::new(heapless::Vec::new());
/// More aliases than were kept: then any device may be one.
static LOST: AtomicBool = AtomicBool::new(false);

pub(super) fn record_aliases(table: &[u8]) {
    let mut found = [Alias { first: 0, last: 0, alias: 0 }; MAX_ALIASES];
    let (n, lost) = aliases(table, &mut found);
    *ALIASES.lock() = found[..n].iter().copied().collect();
    LOST.store(lost, Ordering::Release);
}

/// Whether requester id `id` may share its entry with another id.
pub fn amd_iommu_aliased(id: u16) -> bool {
    LOST.load(Ordering::Acquire) || aliased(&ALIASES.lock(), id)
}
