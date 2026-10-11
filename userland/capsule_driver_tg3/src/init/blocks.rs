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

//! Every send and receive block on, in tg3_reset_hw's order, then the
//! transmitter. The receiver is turned on by rx_filter.rs.

use crate::constants::blocks::*;
use crate::constants::mac::MAC_TX_MODE;
use crate::constants::timeouts::STOP_BLOCK_MS;
use crate::regs::Regs;

use super::wait;

/// The 40 us and 100 us udelays after each flushed mode write.
const SETTLE_MS: u64 = 1;

/// A block's enable bit with its attention bit.
pub const ON: u32 = MODE_ENABLE | MODE_ATTN_ENABLE;

/// A flushed write, then the hold Linux takes after it.
pub fn settle(write: impl FnOnce()) {
    write();
    wait::hold(SETTLE_MS);
}

/// RCVDBDI_MODE: inverted ring size, without the large ring the 57765
/// class lacks.
pub const RCVDBDI_ON: u32 = MODE_ENABLE | RCVDBDI_MODE_INV_RING_SZ;

/*
 * SAFETY: `regs` carries the broker MmioMap base for BAR0 and each offset
 * is a 32-bit block mode register inside it.
 */
pub fn enable(regs: &Regs, tx_mode: u32) {
    unsafe {
        regs.w32(RCVDCC_MODE, ON);
        regs.w32(SNDDATAC_MODE, MODE_ENABLE);
        regs.w32(SNDBDC_MODE, ON);
        regs.w32(RCVBDI_MODE, ON);
        regs.w32(RCVDBDI_MODE, RCVDBDI_ON);
        // Linux writes the plain enable, then the HW_TSO bit with it.
        regs.w32(SNDDATAI_MODE, MODE_ENABLE);
        regs.w32(SNDDATAI_MODE, MODE_ENABLE | SNDDATAI_MODE_HW_TSO);
        regs.w32(SNDBDI_MODE, ON);
        regs.w32(SNDBDS_MODE, ON);
        settle(|| regs.w32f(MAC_TX_MODE, tx_mode));
    }
}

/// tg3_stop_block: the enable bit cleared, then waited off.
///
/// SAFETY for both accesses: `regs` carries the broker MmioMap base for
/// BAR0 and `at` is a 32-bit mode register inside it.
pub fn stop(regs: &Regs, at: usize, bit: u32) {
    unsafe { regs.w32f(at, regs.r32(at) & !bit) };
    let _ = wait::until(STOP_BLOCK_MS, || unsafe { regs.r32(at) & bit == 0 });
}
