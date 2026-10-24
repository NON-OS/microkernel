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

//! Transmit queue 0: the queue disabled, length, base, head and tail zero,
//! then thresholds and the enable in one write. The enable is read back on
//! the clock, so a queue that never started is named in the log instead of
//! showing up as lost frames.

use core::sync::atomic::{fence, Ordering};

use crate::bring::wait::until;
use crate::facts::bounds::QUEUE_ENABLE_MS;
use crate::facts::regs::{
    REG_STATUS, REG_TDBAH, REG_TDBAL, REG_TDH, REG_TDLEN, REG_TDT, REG_TXDCTL,
};
use crate::facts::tx::{TXDCTL_HTHRESH, TXDCTL_PTHRESH, TXDCTL_QUEUE_ENABLE, TXDCTL_WTHRESH};
use crate::queue::layout::TxDesc;
use crate::queue::TxRing;
use crate::regs::Regs;
use crate::sizes::ring::{TX_DESC_COUNT, TX_RING_BYTES};

/// Quotes QUEUE_ENABLE_MS; igb_proofs holds the two equal.
pub const TX_NOT_ENABLED: &str = "tx queue enable did not read back in 100 ms";

pub fn program(regs: &Regs, tx: &TxRing, ring_phys: u64) -> Result<(), &'static str> {
    // SAFETY: `tx.ring_user_va` is the broker DMA grant for the ring,
    // TX_DESC_COUNT descriptors long; `regs` carries a valid broker MmioMap
    // base for BAR0 and every offset is a register inside it (facts::regs).
    unsafe {
        let descs = tx.ring_user_va as *mut TxDesc;
        for i in 0..TX_DESC_COUNT {
            *descs.add(i) = TxDesc::default();
        }
        // The ring was written with plain stores; the part reads it from here.
        fence(Ordering::Release);
        regs.w32(REG_TXDCTL, 0);
        let _ = regs.r32(REG_STATUS);
        regs.w32(REG_TDLEN, TX_RING_BYTES as u32);
        regs.w32(REG_TDBAL, ring_phys as u32);
        regs.w32(REG_TDBAH, (ring_phys >> 32) as u32);
        regs.w32(REG_TDH, 0);
        regs.w32(REG_TDT, 0);
        let txdctl = TXDCTL_PTHRESH | TXDCTL_HTHRESH | TXDCTL_WTHRESH | TXDCTL_QUEUE_ENABLE;
        regs.w32(REG_TXDCTL, txdctl);
    }
    // SAFETY: as above.
    let on = || unsafe { regs.r32(REG_TXDCTL) } & TXDCTL_QUEUE_ENABLE != 0;
    if until(QUEUE_ENABLE_MS, on) {
        Ok(())
    } else {
        Err(TX_NOT_ENABLED)
    }
}
