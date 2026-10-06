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

use super::wait_for;
use crate::log::Line;
use crate::regs::Regs;

/// ERIDR and ERIAR (8168c and later): the extended register window.
const REG_ERIDR: usize = 0x70;
const REG_ERIAR: usize = 0x74;
const ERIAR_FLAG: u32 = 1 << 31;
/// Byte enables, ERIAR bits 12..15 (Linux ERIAR_MASK_*).
pub const ERIAR_MASK_0001: u32 = 0x1 << 12;
pub const ERIAR_MASK_0011: u32 = 0x3 << 12;
pub const ERIAR_MASK_1111: u32 = 0xF << 12;
/// Linux polls ERIAR 100 times 100 us apart.
const ERI_MS: u64 = 10;

fn timed_out(addr: u32) -> &'static str {
    Line::new("rtl8169: ERI access to ").hex(addr).text(" did not complete in 10 ms").send();
    "rtl8169 ERI timeout"
}

/// Linux rtl_eri_write (ERIAR_EXGMAC): data first, then the command with
/// FLAG set, which the part clears when it has taken the write.
pub fn eri_write(regs: &Regs, addr: u32, mask: u32, val: u32) -> Result<(), &'static str> {
    // SAFETY (each block): ERIDR and ERIAR lie inside every mapped window.
    unsafe {
        regs.w32(REG_ERIDR, val);
        regs.w32(REG_ERIAR, ERIAR_FLAG | mask | addr);
    }
    // SAFETY: as above.
    let done = wait_for(ERI_MS, false, || unsafe { regs.r32(REG_ERIAR) } & ERIAR_FLAG != 0);
    done.then_some(()).ok_or_else(|| timed_out(addr))
}

/// Linux rtl_eri_read: the command with FLAG clear; the part sets FLAG
/// once ERIDR holds the dword.
pub fn eri_read(regs: &Regs, addr: u32) -> Result<u32, &'static str> {
    // SAFETY: as above.
    unsafe { regs.w32(REG_ERIAR, ERIAR_MASK_1111 | addr) };
    // SAFETY: as above.
    let done = wait_for(ERI_MS, true, || unsafe { regs.r32(REG_ERIAR) } & ERIAR_FLAG != 0);
    if !done {
        return Err(timed_out(addr));
    }
    // SAFETY: as above.
    Ok(unsafe { regs.r32(REG_ERIDR) })
}

/// Linux rtl_w0w1_eri: read, set `set`, clear `clear`, write all four bytes.
pub fn eri_modify(regs: &Regs, addr: u32, set: u32, clear: u32) -> Result<(), &'static str> {
    let val = eri_read(regs, addr)?;
    eri_write(regs, addr, ERIAR_MASK_1111, (val & !clear) | set)
}
