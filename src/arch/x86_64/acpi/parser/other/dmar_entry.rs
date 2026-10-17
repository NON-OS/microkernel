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

//! One DMAR remapping structure, kept by its type.

use core::mem;
use core::ptr;

use super::dmar_rmrr::RMRR_TYPE;
use crate::arch::x86_64::acpi::tables::Drhd;

const DRHD_TYPE: u16 = 0;

/// Keep the structure at `cursor` (`bytes`, exactly its length). True when
/// it was a segment 0 remapping unit.
///
/// # Safety
/// `cursor` is the directmapped address of `bytes`.
pub(super) unsafe fn entry(kind: u16, cursor: u64, bytes: &[u8]) -> bool {
    let mut found_drhd = false;
    let length = bytes.len();
    match kind {
        RMRR_TYPE => super::dmar_reserved::record(bytes),
        DRHD_TYPE if length >= mem::size_of::<Drhd>() => {
            // SAFETY: `cursor` maps `bytes`, at least a Drhd long.
            let drhd = unsafe { ptr::read_volatile(cursor as *const Drhd) };
            let (segment, base) = (drhd.segment, drhd.register_base_address);
            found_drhd |= super::dmar_units::record(bytes, segment, base);
        }
        _ => {}
    }
    found_drhd
}
