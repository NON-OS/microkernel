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

//! The part stopped and reset, in the order the 8254x needs on silicon:
//!
//! 1. Mask every cause and stop both DMA engines, then give bus-master
//!    cycles already in flight time to drain. Firmware (PXE, UEFI UNDI) or a
//!    previous instance can leave RCTL.EN set, and a reset landing mid-DMA
//!    is a known hang on PCI-X parts.
//! 2. Set CTRL.RST and poll for it to self-clear, reading nothing in the
//!    first microsecond the manual says the part is unreachable.
//! 3. Wait out the EEPROM auto-load the reset starts. It rewrites RAL0/RAH0
//!    and parts of CTRL, so a MAC or SLU written before it finishes can be
//!    put back to the factory value: unicast then goes to the wrong filter.
//!    Then hold the part to both engines off, as the reset leaves them.
//!
//! A teardown is this and nothing more: it wants no link set up.

use crate::constants::regs::{REG_CTRL, REG_IMC, REG_RCTL, REG_STATUS, REG_TCTL};
use crate::constants::status::{CTRL_RST, RCTL_EN, TCTL_EN};
use crate::regs::Regs;

use super::hold::{hold_ms, until};

/// Linux e1000_reset_hw's drain; the microsecond after RST is set when the
/// part is unreachable; the bound on RST self-clearing, which takes
/// microseconds on a working part.
const DMA_DRAIN_MS: u64 = 10;
const RST_SETTLE_MS: u64 = 1;
const RST_CLEAR_MS: u64 = 50;
/// EEPROM auto-load after a global reset: 5 ms on 82540/82545/82546,
/// 20 ms on 82541/82547, so the longer one covers every listed part.
const EEPROM_RELOAD_MS: u64 = 20;

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY (each block): `regs` carries a base from a valid broker MmioMap
    // grant; offsets are 32-bit aligned per the 8254x manual.
    unsafe {
        // Both engines off and nothing else: a card that never gets a
        // station address never has an enable bit written.
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        regs.w32(REG_RCTL, 0);
        regs.w32(REG_TCTL, 0);
        let _ = regs.r32(REG_STATUS);
    }
    hold_ms(DMA_DRAIN_MS);
    // SAFETY: as above.
    unsafe { regs.w32(REG_CTRL, regs.r32(REG_CTRL) | CTRL_RST) };
    hold_ms(RST_SETTLE_MS);
    // SAFETY: as above.
    if !until(RST_CLEAR_MS, || unsafe { regs.r32(REG_CTRL) } & CTRL_RST == 0) {
        return Err("CTRL.RST did not self-clear");
    }
    hold_ms(EEPROM_RELOAD_MS);
    // SAFETY: as above.
    let running = unsafe { (regs.r32(REG_RCTL) & RCTL_EN) | (regs.r32(REG_TCTL) & TCTL_EN) };
    if running != 0 {
        return Err("RCTL/TCTL still on after reset");
    }
    Ok(())
}
