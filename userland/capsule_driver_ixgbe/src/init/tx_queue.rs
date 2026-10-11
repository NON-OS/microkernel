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

//! ixgbe_configure_tx: the DMA transmit engine on (DMATXCTL.TE, which the
//! 82599 and later need before a queue is enabled), then queue 0 as
//! ixgbe_configure_tx_ring has it: disabled and flushed, base, length, head
//! and tail zero, then thresholds and the enable in one write, read back on
//! the clock.

use core::sync::atomic::{fence, Ordering};

use crate::constants::ctrl::DMATXCTL_TE;
use crate::constants::queue::{TX_DESC_COUNT, TX_RING_BYTES};
use crate::constants::regs::{REG_DMATXCTL, REG_STATUS, REG_TDBAH, REG_TDBAL, REG_TDH};
use crate::constants::regs::{REG_TDLEN, REG_TDT, REG_TXDCTL};
use crate::constants::timeouts::QUEUE_ENABLE_MS;
use crate::constants::tx_bits::{TXDCTL_ENABLE, TXDCTL_HTHRESH, TXDCTL_PTHRESH, TXDCTL_WTHRESH};
use crate::init::wait::until;
use crate::queue::layout::TxDesc;
use crate::queue::TxRing;
use crate::regs::Regs;

/// Quotes QUEUE_ENABLE_MS; ixgbe_proofs holds the two equal.
pub const TX_NOT_ENABLED: &str = "tx queue enable did not read back in 20 ms";

pub fn program(regs: &Regs, tx: &TxRing, ring_phys: u64) -> Result<(), &'static str> {
    // SAFETY: `tx.ring_user_va` is the broker DMA grant for the ring,
    // TX_DESC_COUNT descriptors long; `regs` carries a valid broker MmioMap
    // base for BAR0 and every offset is a register inside it (ixgbe_type.h).
    unsafe {
        let descs = tx.ring_user_va as *mut TxDesc;
        for i in 0..TX_DESC_COUNT {
            *descs.add(i) = TxDesc::default();
        }
        // The ring was written with plain stores; the part reads it from here.
        fence(Ordering::Release);
        let dmatxctl = regs.r32(REG_DMATXCTL);
        regs.w32(REG_DMATXCTL, dmatxctl | DMATXCTL_TE);
        regs.w32(REG_TXDCTL, 0);
        let _ = regs.r32(REG_STATUS);
        regs.w32(REG_TDBAL, ring_phys as u32);
        regs.w32(REG_TDBAH, (ring_phys >> 32) as u32);
        regs.w32(REG_TDLEN, TX_RING_BYTES as u32);
        regs.w32(REG_TDH, 0);
        regs.w32(REG_TDT, 0);
        let txdctl = TXDCTL_PTHRESH | TXDCTL_HTHRESH | TXDCTL_WTHRESH | TXDCTL_ENABLE;
        regs.w32(REG_TXDCTL, txdctl);
    }
    // SAFETY: as above.
    let on = || unsafe { regs.r32(REG_TXDCTL) } & TXDCTL_ENABLE != 0;
    if until(QUEUE_ENABLE_MS, on) {
        Ok(())
    } else {
        Err(TX_NOT_ENABLED)
    }
}
