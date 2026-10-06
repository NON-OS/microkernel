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

//! Before the reset nothing new may start and nothing in flight may be cut
//! off. Linux e1000e_reset flushes the I219 rings (pch_spt and later), then
//! reset_hw_ich8lan / reset_hw_82571 block new PCIe master requests
//! (e1000e_disable_pcie_master), mask every cause, stop both units and wait
//! out the transactions already issued. Firmware (PXE, UEFI UNDI) can leave
//! the receiver running into memory this driver never gave it.

use crate::constants::ctrl::CTRL_GIO_MASTER_DISABLE;
use crate::constants::regs::{REG_CTRL, REG_IMC, REG_RCTL, REG_STATUS, REG_TCTL};
use crate::constants::rxtx::{TCTL_EN, TCTL_PSP};
use crate::constants::status::STATUS_GIO_MASTER_ENABLE;
use crate::constants::timeouts::{DMA_DRAIN_MS, MASTER_DISABLE_MS};
use crate::constants::Family;
use crate::log::say;
use crate::setup::Driver;
use crate::wait::{idle_until, sleep_ms};

use super::flush_rings;

pub fn run(d: &Driver) {
    let regs = &d.regs;
    // SAFETY: `regs` is the broker-mapped BAR0 window; every offset in this
    // file is a 4-byte register inside it on every family.
    unsafe { regs.w32(REG_IMC, 0xFFFF_FFFF) };
    if d.family >= Family::PchSpt {
        flush_rings::run(d);
    }
    // SAFETY: as above.
    unsafe { regs.modify(REG_CTRL, 0, CTRL_GIO_MASTER_DISABLE) };
    // SAFETY: as above.
    let idle = || unsafe { regs.r32(REG_STATUS) } & STATUS_GIO_MASTER_ENABLE == 0;
    if !idle_until(MASTER_DISABLE_MS, idle) {
        // Linux logs this and resets anyway; so does this driver.
        say("PCIe master requests still pending after 80 ms, resetting anyway");
    }
    // SAFETY: as above.
    unsafe {
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        regs.w32(REG_RCTL, 0);
        if d.family.is_pch() {
            regs.w32(REG_TCTL, TCTL_PSP);
        } else {
            regs.modify(REG_TCTL, TCTL_EN, 0);
        }
        let _ = regs.r32(REG_STATUS);
    }
    sleep_ms(DMA_DRAIN_MS);
}
