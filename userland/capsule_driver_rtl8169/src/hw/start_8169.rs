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

use super::regs::{REG_CONFIG2, REG_EARLY_TX_THRES};
use crate::chip::MacVersion;
use crate::regs::Regs;

/// EarlyTxThres "no early transmit" (Linux NoEarlyTx).
const NO_EARLY_TX: u8 = 0x3F;
/// The register rtl8169_set_magic_reg writes, and Config2's 66 MHz bit.
const REG_MAGIC: usize = 0x7C;
const PCI_CLOCK_66MHZ: u8 = 0x01;

/// Linux rtl_hw_start_8169 for the PCI RTL8169/8110 (VER_02 to VER_06).
pub fn start_8169(regs: &Regs, ver: MacVersion) {
    // SAFETY (each block): EarlyTxThres, Config2 and 0x7C lie below 0x100,
    // inside every mapped window.
    unsafe { regs.w8(REG_EARLY_TX_THRES, NO_EARLY_TX) };
    // rtl8169_set_magic_reg: the 8169sc (VER_05, VER_06) only.
    let magic = match ver.0 {
        5 => 0x000F_FF00,
        6 => 0x00FF_FF00,
        _ => return,
    };
    // SAFETY: as above.
    let clock_66 = unsafe { regs.r8(REG_CONFIG2) } & PCI_CLOCK_66MHZ != 0;
    let magic = if clock_66 { magic | 0xFF } else { magic };
    // SAFETY: as above.
    unsafe { regs.w32(REG_MAGIC, magic) };
}
