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

//! Which physical ranges the kernel copies a firmware table from. The
//! address comes from firmware (the RSDT or XSDT, the FADT, the SMBIOS
//! entry point), so a broken or hostile one could name RAM the allocator
//! owns, the kernel's own image, or device space, and the copy would hand
//! it to the capsule. Only firmware's own memory is read, or the legacy
//! BIOS window, where SMBIOS 2 tables live and which the map may call RAM.

use super::facts::Facts;
use super::memory::span;
use super::refusal::Refusal;

/// 0xF0000-0xFFFFF, the legacy BIOS area (DSP0134 5.2.1; ACPI 6.5 5.2.5.1).
pub const LEGACY_BIOS: (u64, u64) = (0xF_0000, 0x10_0000);

/// Every byte of [base, base + len) outside the legacy window must be
/// memory the map types as firmware's (`holds_tables`).
pub fn table_read(base: u64, len: u64, f: &impl Facts) -> Result<(), Refusal> {
    let end = span(base, len)?;
    let (lo, hi) = LEGACY_BIOS;
    let pieces = [(base, end.min(lo)), (base.max(hi), end)];
    let mut bad = None;
    for (b, e) in pieces.into_iter().filter(|(b, e)| b < e) {
        f.mem_kinds(b, e, &mut |k| {
            if !k.holds_tables() {
                bad.get_or_insert(k);
            }
        });
    }
    bad.map_or(Ok(()), |k| Err(Refusal::Memory(k)))
}
