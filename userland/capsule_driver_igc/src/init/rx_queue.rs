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

//! igc_configure_rx_ring for queue 0, one buffer per advanced descriptor:
//! queue disabled, base, length, head and tail zero, SRRCTL (2 KiB packet
//! buffer, 256-byte header size, ADV_ONEBUF), then thresholds and
//! QUEUE_ENABLE. igc sets no SRRCTL drop-enable bit for its queues and
//! neither does this driver. The enable is read back on the clock; only then
//! does RDT hand the part every slot but one, as igc_alloc_rx_buffers does.

use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{RX_DESC_COUNT, RX_RING_BYTES};
use crate::constants::regs::*;
use crate::constants::rx_bits::*;
use crate::constants::timeouts::QUEUE_ENABLE_MS;
use crate::init::wait::until;
use crate::queue::layout::RxDesc;
use crate::queue::RxRing;
use crate::regs::Regs;

/// Quotes QUEUE_ENABLE_MS; igc_proofs holds the two equal.
pub const RX_NOT_ENABLED: &str = "rx queue enable did not read back in 100 ms";

const SRRCTL_FIELDS: u32 = SRRCTL_BSIZEPKT_MASK | SRRCTL_BSIZEHDR_MASK | SRRCTL_DESCTYPE_MASK;
const SRRCTL_ONEBUF: u32 = SRRCTL_BSIZEHDR_256 | SRRCTL_BSIZEPKT_2K | SRRCTL_DESCTYPE_ADV_ONEBUF;
const RXDCTL_ON: u32 = RXDCTL_PTHRESH | RXDCTL_HTHRESH | RXDCTL_WTHRESH | RXDCTL_QUEUE_ENABLE;

pub fn program(regs: &Regs, rx: &RxRing, ring_phys: u64) -> Result<(), &'static str> {
    // SAFETY: `rx.ring_user_va` is the broker DMA grant for the ring,
    // RX_DESC_COUNT descriptors long; `regs` carries a valid broker MmioMap
    // base for BAR0 and every offset is a register inside it (igc_regs.h).
    unsafe {
        let descs = rx.ring_user_va as *mut RxDesc;
        for i in 0..RX_DESC_COUNT {
            *descs.add(i) = RxDesc { buffer_addr: rx.buffer_phys(i as u16), ..RxDesc::default() };
        }
        fence(Ordering::Release);
        regs.w32(REG_RXDCTL, 0);
        regs.w32(REG_RDBAL, ring_phys as u32);
        regs.w32(REG_RDBAH, (ring_phys >> 32) as u32);
        regs.w32(REG_RDLEN, RX_RING_BYTES as u32);
        regs.w32(REG_RDH, 0);
        regs.w32(REG_RDT, 0);
        let srrctl = regs.r32(REG_SRRCTL) & !SRRCTL_FIELDS;
        regs.w32(REG_SRRCTL, srrctl | SRRCTL_ONEBUF);
        regs.w32(REG_RXDCTL, RXDCTL_ON);
    }
    // SAFETY: as above.
    let on = || unsafe { regs.r32(REG_RXDCTL) } & RXDCTL_QUEUE_ENABLE != 0;
    if !until(QUEUE_ENABLE_MS, on) {
        return Err(RX_NOT_ENABLED);
    }
    // SAFETY: as above.
    unsafe { regs.w32(REG_RDT, RX_DESC_COUNT as u32 - 1) };
    Ok(())
}
