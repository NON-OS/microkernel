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

//! RX bring-up: zero the ring, prime every slot with its buffer, select the
//! legacy descriptor (RFCTL.EXTEN clear, RCTL.DTYP 00), program the ring
//! registers with the tail on the last slot, and enable the receiver with
//! broadcast, 2048-byte buffers and the CRC stripped.

use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{RX_DESC_COUNT, RX_RING_BYTES};
use crate::constants::regs::{
    REG_RCTL, REG_RDBAH, REG_RDBAL, REG_RDH, REG_RDLEN, REG_RDT, REG_RFCTL,
};
use crate::constants::rxtx::{RCTL_BAM, RCTL_EN, RCTL_SECRC, RCTL_SZ_2048, RFCTL_EXTEN};
use crate::queue::layout::RxDesc;
use crate::queue::RxRing;
use crate::regs::Regs;

pub fn program(regs: &Regs, rx: &RxRing, ring_phys: u64) {
    // SAFETY: `rx.ring_user_va` is the RX ring DMA grant, RX_DESC_COUNT
    // descriptors long; the registers are 4-byte offsets in BAR0.
    unsafe {
        let descs = rx.ring_user_va as *mut RxDesc;
        for i in 0..RX_DESC_COUNT {
            let d = &mut *descs.add(i);
            *d = RxDesc::default();
            d.buffer_addr = rx.buffer_phys(i as u16);
        }
        // The ring was written with plain stores; the part reads it from here on.
        fence(Ordering::Release);
        regs.modify(REG_RFCTL, RFCTL_EXTEN, 0);
        regs.w32(REG_RDBAL, ring_phys as u32);
        regs.w32(REG_RDBAH, (ring_phys >> 32) as u32);
        regs.w32(REG_RDLEN, RX_RING_BYTES as u32);
        regs.w32(REG_RDH, 0);
        regs.w32(REG_RDT, (RX_DESC_COUNT as u32) - 1);
        regs.w32(REG_RCTL, RCTL_EN | RCTL_BAM | RCTL_SZ_2048 | RCTL_SECRC);
    }
}
