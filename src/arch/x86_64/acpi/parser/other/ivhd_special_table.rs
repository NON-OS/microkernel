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

//! The IVHD special entries, kept from boot for AMD interrupt remapping.

use spin::Mutex;

use super::ivhd_special::{specials, Special, IOAPIC, MAX_SPECIALS};

static SPECIALS: Mutex<heapless::Vec<Special, MAX_SPECIALS>> = Mutex::new(heapless::Vec::new());

pub(super) fn record(ivrs: &[u8]) {
    let mut found = [Special { variety: 0, handle: 0, id: 0 }; MAX_SPECIALS];
    let n = specials(ivrs, &mut found);
    *SPECIALS.lock() = found[..n].iter().copied().collect();
}

/// The requester id of the IOAPIC whose MADT id is `ioapic_id`, if IVRS
/// names it.
pub fn amd_ioapic_requester(ioapic_id: u8) -> Option<u16> {
    let all = SPECIALS.lock();
    all.iter().find(|s| s.variety == IOAPIC && s.handle == ioapic_id).map(|s| s.id)
}
