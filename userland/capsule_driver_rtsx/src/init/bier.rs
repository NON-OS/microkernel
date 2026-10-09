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

//! Which events the chip raises in BIPR (rtsx_pci_enable_bus_int): command
//! and DMA ends, card changes on both slots, and over-current on the
//! RTS522A. The PCI interrupt pin stays disabled; the driver polls BIPR.

use crate::chip::Family;
use crate::regs::host::{
    BIER, MS_INT_EN, SD_INT_EN, SD_OC_INT_EN, TRANS_FAIL_INT_EN, TRANS_OK_INT_EN,
};
use crate::setup::Driver;

pub fn enable_bus_int(drv: &Driver) {
    // Two slots (SD and MemoryStick) on the RTS5227 family: num_slots = 2.
    let mut bier = TRANS_OK_INT_EN | TRANS_FAIL_INT_EN | SD_INT_EN | MS_INT_EN;
    if drv.family == Family::Rts522a {
        bier |= SD_OC_INT_EN;
    }
    drv.regs.w32(BIER, bier);
}
