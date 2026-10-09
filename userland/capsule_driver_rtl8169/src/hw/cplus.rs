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

use crate::chip::MacVersion;

/// CPlusCmd bits Linux keeps from what the part holds at probe (CPCMD_MASK
/// less the two offloads): Normal_mode and the interrupt timer.
const NORMAL_MODE: u16 = 1 << 13;
const INTT_MASK: u16 = 0b11;
/// PCI multiple read/write, and the analog PLL the first 8169s need.
const PCI_MUL_RW: u16 = 1 << 3;
const EN_ANA_PLL: u16 = 1 << 14;

/*
 * Linux rtl_init_one keeps `CPlusCmd & CPCMD_MASK`, and rtl_hw_start_8169
 * adds PCIMulRW (and EnAnaPLL on VER_02 and VER_03). RxVlan and RxChkSum,
 * the other two CPCMD_MASK bits, stay off: this driver hands frames up
 * whole, and RxVlan would strip the 802.1Q tag into a descriptor word it
 * never reads.
 */
pub fn cplus_cmd(held: u16, ver: MacVersion) -> u16 {
    let mut cmd = held & (NORMAL_MODE | INTT_MASK);
    if ver.is_8169() {
        cmd |= PCI_MUL_RW;
        if ver.0 <= 3 {
            cmd |= EN_ANA_PLL;
        }
    }
    cmd
}
