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

use core::mem::size_of;

use crate::arch::x86_64::acpi::tables::{table_span, SdtHeader};
use crate::memory::addr::PhysAddr;
use crate::memory::layout::DIRECTMAP_SIZE;

pub(crate) fn directmap(phys: u64) -> Option<u64> {
    crate::memory::unified::phys_to_virt_checked(PhysAddr::new(phys)).map(|v| v.as_u64())
}

/// The directmapped table at `phys` and its header, when its whole declared
/// length is mapped and within MAX_TABLE_BYTES (`table_span`, K11).
pub(crate) fn mapped_table(phys: u64) -> Option<(u64, SdtHeader)> {
    let head = size_of::<SdtHeader>();
    table_span(phys, head as u32, head, DIRECTMAP_SIZE)?;
    let virt = directmap(phys)?;
    // SAFETY: the header's bytes lie inside the direct map, checked above.
    let header = unsafe { core::ptr::read_volatile(virt as *const SdtHeader) };
    table_span(phys, header.length, head, DIRECTMAP_SIZE)?;
    Some((virt, header))
}
