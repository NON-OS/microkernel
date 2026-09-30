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

//! Hardware reset + IRQ quiesce + link bring-up, in the order the 8254x
//! needs on silicon:
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
//! 4. Mask again, clear latched causes, and bring the link up.

use nonos_libc::Deadline;

use crate::constants::regs::{REG_CTRL, REG_ICR, REG_IMC, REG_RCTL, REG_STATUS, REG_TCTL};
use crate::constants::status::{CTRL_ASDE, CTRL_LRST, CTRL_RST, CTRL_SLU};
use crate::regs::Regs;

/// Linux e1000_reset_hw's drain before the reset.
const DMA_DRAIN_MS: u64 = 10;
/// The part is not addressable for about a microsecond after RST is set.
const RST_SETTLE_MS: u64 = 1;
/// Bound on RST self-clearing; it takes microseconds on a working part.
const RST_CLEAR_MS: u64 = 50;
/// EEPROM auto-load after a global reset: 5 ms on 82540/82545/82546,
/// 20 ms on 82541/82547, so the longer one covers every listed part.
const EEPROM_RELOAD_MS: u64 = 20;

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: eK@nonos.systems — `regs` carries a base from a
    // valid broker MmioMap grant; offsets are 32-bit aligned per
    // the 8254x manual.
    unsafe {
        // Both engines off. Nothing else is set here: a card that never gets a
        // station address is left with neither enable bit ever written.
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        regs.w32(REG_RCTL, 0);
        regs.w32(REG_TCTL, 0);
        let _ = regs.r32(REG_STATUS);
        hold_ms(DMA_DRAIN_MS);

        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_RST);
        hold_ms(RST_SETTLE_MS);
        let deadline = Deadline::after_ms(RST_CLEAR_MS);
        while regs.r32(REG_CTRL) & CTRL_RST != 0 {
            if deadline.expired() {
                return Err("CTRL.RST did not self-clear");
            }
            core::hint::spin_loop();
        }
        hold_ms(EEPROM_RELOAD_MS);

        regs.w32(REG_IMC, 0xFFFF_FFFF);
        let _ = regs.r32(REG_ICR);
        let mut ctrl = regs.r32(REG_CTRL);
        ctrl &= !CTRL_LRST;
        ctrl |= CTRL_SLU | CTRL_ASDE;
        regs.w32(REG_CTRL, ctrl);
    }
    Ok(())
}

// At least `ms` milliseconds: uptime counts whole milliseconds, so a deadline
// `ms` ahead can fall due up to one early.
fn hold_ms(ms: u64) {
    let until = Deadline::after_ms(ms + 1);
    while !until.expired() {
        core::hint::spin_loop();
    }
}
