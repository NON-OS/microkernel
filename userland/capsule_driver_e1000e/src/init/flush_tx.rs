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

//! Linux e1000_flush_tx_ring: give the transmitter one more descriptor, a
//! 512-byte frame of whatever the ring memory holds, so it drains.
//!
//! One difference, forced by where this runs: Linux appends that descriptor
//! to its own ring, already programmed. Here the pending ring is whatever
//! firmware left, in memory this capsule does not own and cannot write. So
//! queue 0 is first pointed at this driver's own ring (empty, head and tail
//! zero) and the descriptor goes in its slot 0.

use core::ptr::write_volatile;
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{TX_CMD_IFCS, TX_RING_BYTES};
use crate::constants::regs::{REG_TCTL, REG_TDBAH, REG_TDBAL, REG_TDH, REG_TDLEN, REG_TDT};
use crate::constants::rxtx::TCTL_EN;
use crate::constants::timeouts::SHORT_MS;
use crate::queue::layout::TxDesc;
use crate::setup::Driver;
use crate::wait::sleep_ms;

/// e1000_flush_tx_ring's `size`.
const FLUSH_LEN: u16 = 512;

pub fn run(d: &Driver) {
    let ring = d.tx_ring_device_addr;
    let desc =
        TxDesc { buffer_addr: ring, length: FLUSH_LEN, cmd: TX_CMD_IFCS, ..TxDesc::default() };
    // SAFETY: `ring_user_va` is the TX ring DMA grant taken in setup, at
    // least TX_DESC_COUNT descriptors long; slot 0 is inside it. The
    // registers are 4-byte offsets in the broker-mapped BAR0 window.
    unsafe {
        write_volatile(d.tx.ring_user_va as *mut TxDesc, desc);
        fence(Ordering::Release);
        d.regs.w32(REG_TDBAL, ring as u32);
        d.regs.w32(REG_TDBAH, (ring >> 32) as u32);
        d.regs.w32(REG_TDLEN, TX_RING_BYTES as u32);
        d.regs.w32(REG_TDH, 0);
        d.regs.w32(REG_TDT, 0);
        d.regs.modify(REG_TCTL, 0, TCTL_EN);
        d.regs.w32(REG_TDT, 1);
    }
    // Linux waits 200 to 250 us; a millisecond is this clock's shortest.
    sleep_ms(SHORT_MS);
}
