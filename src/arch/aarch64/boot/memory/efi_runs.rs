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

//! The free RAM a NONOS loader's EFI memory map names. Only Conventional
//! memory (UEFI type 7) counts, as on x86_64: the loader's own data, the
//! runtime services and everything firmware keeps stay out. Of that, only
//! write-back memory (EFI_MEMORY_WB), since the direct map maps RAM
//! cacheable, and none set aside for a specific purpose (EFI_MEMORY_SP).
//! Pure, so it is held on the host (boot_memory_proofs).

use super::usable::Usable;

/// UEFI's EfiConventionalMemory.
pub(crate) const CONVENTIONAL: u32 = 7;
/// EFI_MEMORY_WB and EFI_MEMORY_SP, UEFI's memory attribute bits.
pub(crate) const WB: u64 = 1 << 3;
pub(crate) const SP: u64 = 1 << 18;
const PAGE: u64 = 0x1000;

/// The free runs of a map given as (type, start, pages, attributes), each
/// page aligned inward; an entry whose end does not fit a u64 is left out.
pub(crate) fn from_efi(entries: impl Iterator<Item = (u32, u64, u64, u64)>) -> Usable {
    let mut u = Usable::EMPTY;
    let free = |e: &(u32, u64, u64, u64)| e.0 == CONVENTIONAL && e.3 & WB != 0 && e.3 & SP == 0;
    for (_, start, pages, _) in entries.filter(free) {
        let end = pages.checked_mul(PAGE).and_then(|len| start.checked_add(len));
        let (Some(end), Some(first)) = (end, start.checked_add(PAGE - 1)) else {
            continue;
        };
        u.add(first & !(PAGE - 1), end & !(PAGE - 1));
    }
    u
}
