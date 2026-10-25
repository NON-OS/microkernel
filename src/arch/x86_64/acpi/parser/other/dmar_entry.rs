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

use super::dmar_ats::{ATSR_TYPE, SATC_TYPE};
use super::dmar_namespace_store::{record_andd, ANDD_TYPE};
use super::dmar_rmrr::RMRR_TYPE;
use super::dmar_rmrr_store::record_rmrr;
use crate::arch::x86_64::acpi::tables::Drhd;

const DRHD_TYPE: u16 = 0;

/// Keep the whole structure `bytes` of type `kind`. True when it was a
/// segment 0 remapping unit the kernel kept.
pub(super) fn entry(kind: u16, bytes: &[u8]) -> bool {
    match kind {
        DRHD_TYPE if bytes.len() >= mem::size_of::<Drhd>() => {
            // SAFETY: eK@nonos.systems - `bytes` holds a whole Drhd, packed.
            let drhd = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const Drhd) };
            let (segment, base) = (drhd.segment, drhd.register_base_address);
            return super::dmar_units::record_drhd(bytes, segment, base);
        }
        RMRR_TYPE => record_rmrr(bytes),
        ANDD_TYPE => record_andd(bytes),
        ATSR_TYPE | SATC_TYPE if bytes.len() >= 8 => super::dmar_ats::record(kind, bytes),
        _ => {}
    }
    false
}
