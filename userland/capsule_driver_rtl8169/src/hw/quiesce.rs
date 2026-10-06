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

use super::conds::{npq_busy, tx_fifo_empty};
use super::regs::RX_CONFIG_ACCEPT_MASK;
use super::{enable_rxdvgate, hold_ms, request_stop, wait_for};
use crate::chip::MacVersion;
use crate::constants::regs::REG_RX_CONFIG;
use crate::log::Line;
use crate::regmap::mask_and_ack;
use crate::regs::Regs;

/*
 * Linux rtl8169_cleanup, the part that touches the chip: mask and ack, stop
 * accepting frames, and let DMA stop the way each version needs before
 * rtl_hw_reset. Firmware (PXE, a UEFI driver) can leave either engine
 * running, and a reset that lands mid-DMA can wedge the part.
 */
pub fn quiesce(regs: &Regs, ver: MacVersion) {
    mask_and_ack(regs, ver);
    // SAFETY: RxConfig (0x44) lies inside every mapped window.
    unsafe { regs.w32(REG_RX_CONFIG, regs.r32(REG_RX_CONFIG) & !RX_CONFIG_ACCEPT_MASK) };
    let stopped = match ver.0 {
        // 20 us x 2000 polls for the 8168dp's normal queue to go idle.
        28 | 31 => wait_for(40, false, || npq_busy(regs)).then_some(()).ok_or("TxPoll NPQ"),
        // StopReq, then 100 us x 666 polls for the TX FIFO.
        34..=38 => {
            request_stop(regs);
            let empty = wait_for(67, true, || tx_fifo_empty(regs));
            empty.then_some(()).ok_or("TxConfig TXCFG_EMPTY")
        }
        40.. => {
            let gated = enable_rxdvgate(regs, ver);
            hold_ms(2);
            gated
        }
        _ => {
            request_stop(regs);
            hold_ms(1);
            Ok(())
        }
    };
    if let Err(what) = stopped {
        Line::new("rtl8169: DMA did not stop before reset (")
            .text(what)
            .text("), resetting anyway")
            .send();
    }
}
