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

//! Where the multi-link capability sits in the controller's register space.

use crate::regs::Regs;

const LLCH: u32 = 0x14;
const CAP_ID_ML: u32 = 0x2;
pub(super) const ML_LCAP: u32 = 0x40;
pub(super) const ML_LCTL: u32 = 0x44;
const MAX_CAPS: u32 = 10;

/// The multi-link capability's offset, walking the linked capability list
/// from LLCH as `snd_hdac_bus_parse_capabilities` does. The pointers are the
/// controller's, so the walk ends at one not 4-byte aligned, or whose
/// capability does not lie, through its link control register, inside the
/// `window` the broker mapped.
pub fn multilink(regs: Regs, window: u64) -> Option<u32> {
    // SAFETY: LLCH is a global register, inside the window (`crate::regs`).
    let mut off = unsafe { regs.r16(LLCH) } as u32;
    let mut n = 0u32;
    while off != 0 && off.is_multiple_of(4) && n < MAX_CAPS && (off + ML_LCTL + 4) as u64 <= window
    {
        // SAFETY: the loop holds `off` 4-byte aligned, with the capability
        // through its LCTL inside the mapped window.
        let hdr = unsafe { regs.r32(off) };
        if hdr == u32::MAX {
            return None;
        }
        if (hdr >> 16) & 0xfff == CAP_ID_ML {
            return Some(off);
        }
        off = hdr & 0xffff;
        n += 1;
    }
    None
}
