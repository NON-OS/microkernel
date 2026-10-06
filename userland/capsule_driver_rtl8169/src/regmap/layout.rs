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
use crate::constants::regs::{REG_IMR, REG_ISR, REG_TX_POLL, TX_POLL_NPQ};

/// IntrMask_8125 and IntrStatus_8125: 32 bits wide, at 0x38 and 0x3C.
const REG_IMR_8125: usize = 0x38;
const REG_ISR_8125: usize = 0x3C;
/// TxPoll_8125, 16 bits; bit 0 polls the normal-priority ring.
const REG_TX_POLL_8125: usize = 0x90;
const TX_POLL_8125_NPQ: u16 = 1 << 0;

/// Where the TX doorbell is and what rings it (Linux rtl8169_doorbell).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Doorbell {
    pub offset: usize,
    pub value: u16,
    pub wide: bool,
}

/// The interrupt mask and status registers (Linux rtl_irq_disable,
/// rtl_get_events, rtl_ack_events). `wide` is 32-bit access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventRegs {
    pub imr: usize,
    pub isr: usize,
    pub wide: bool,
}

/*
 * On an 8125, 0x38 is the interrupt mask: the 8168 doorbell byte written
 * there set mask bit 6 (RxFIFOOver) and rang nothing, so no frame ever
 * left, and 0x3C..0x3F is one 32-bit status register, not IMR and ISR.
 */
pub fn doorbell_of(ver: MacVersion) -> Doorbell {
    if ver.is_8125() {
        Doorbell { offset: REG_TX_POLL_8125, value: TX_POLL_8125_NPQ, wide: true }
    } else {
        Doorbell { offset: REG_TX_POLL, value: TX_POLL_NPQ as u16, wide: false }
    }
}

pub fn events_of(ver: MacVersion) -> EventRegs {
    if ver.is_8125() {
        EventRegs { imr: REG_IMR_8125, isr: REG_ISR_8125, wide: true }
    } else {
        EventRegs { imr: REG_IMR, isr: REG_ISR, wide: false }
    }
}
