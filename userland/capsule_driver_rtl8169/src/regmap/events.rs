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

use super::events_of;
use crate::chip::MacVersion;
use crate::constants::regs::REG_CMD;
use crate::regs::Regs;

pub fn read_events(regs: &Regs, ver: MacVersion) -> u32 {
    let ev = events_of(ver);
    // SAFETY: IMR and ISR lie in 0x38..0x40, inside every mapped window.
    unsafe {
        if ev.wide {
            regs.r32(ev.isr)
        } else {
            regs.r16(ev.isr) as u32
        }
    }
}

pub fn read_mask(regs: &Regs, ver: MacVersion) -> u32 {
    let ev = events_of(ver);
    // SAFETY: IMR and ISR lie in 0x38..0x40, inside every mapped window.
    unsafe {
        if ev.wide {
            regs.r32(ev.imr)
        } else {
            regs.r16(ev.imr) as u32
        }
    }
}

/// Clear the status bits given; the register is write-one-to-clear.
pub fn ack(regs: &Regs, ver: MacVersion, bits: u32) {
    let ev = events_of(ver);
    // SAFETY: IMR and ISR lie in 0x38..0x40, inside every mapped window.
    unsafe {
        if ev.wide {
            regs.w32(ev.isr, bits);
        } else {
            regs.w16(ev.isr, bits as u16);
        }
    }
}

/// Linux rtl8169_irq_mask_and_ack: mask every source, clear what latched,
/// and read ChipCmd so the writes are posted before the caller goes on.
pub fn mask_and_ack(regs: &Regs, ver: MacVersion) {
    let ev = events_of(ver);
    // SAFETY: IMR, ISR and ChipCmd lie in 0x37..0x40, inside the window.
    unsafe {
        if ev.wide {
            regs.w32(ev.imr, 0);
        } else {
            regs.w16(ev.imr, 0);
        }
        ack(regs, ver, u32::MAX);
        let _ = regs.r8(REG_CMD);
    }
}
