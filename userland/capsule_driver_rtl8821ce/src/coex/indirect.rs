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

//! The LTE-coex registers sit behind an indirect window (rtw88 util.c,
//! ltecoex_read_reg and ltecoex_reg_write; rtw8821c_ltecoex_addr).

use crate::regs::Mmio;

const CTRL: usize = 0x1700;
const WDATA: usize = 0x1704;
const RDATA: usize = 0x1708;
/// LTECOEX_READY. check_hw_ready waits up to 10 ms; a PCIe register read
/// takes about a microsecond, so 10000 reads wait at least as long.
const READY: u32 = 1 << 29;
const READY_READS: u32 = 10_000;

fn ready<M: Mmio>(mmio: &M) -> bool {
    (0..READY_READS).any(|_| mmio.read32(CTRL) & READY != 0)
}

pub(super) fn read<M: Mmio>(mmio: &M, offset: u16) -> Option<u32> {
    if !ready(mmio) {
        return None;
    }
    mmio.write32(CTRL, 0x800F_0000 | offset as u32);
    Some(mmio.read32(RDATA))
}

pub(super) fn write<M: Mmio>(mmio: &M, offset: u16, value: u32) -> bool {
    if !ready(mmio) {
        return false;
    }
    mmio.write32(WDATA, value);
    mmio.write32(CTRL, 0xC00F_0000 | offset as u32);
    true
}

/// rtw_coex_write_indirect_reg: set the field under `mask` to `val`.
pub(super) fn write_field<M: Mmio>(mmio: &M, offset: u16, mask: u32, val: u32) -> bool {
    let Some(cur) = read(mmio, offset) else {
        return false;
    };
    let shift = mask.trailing_zeros();
    write(mmio, offset, (cur & !mask) | ((val << shift) & mask))
}
