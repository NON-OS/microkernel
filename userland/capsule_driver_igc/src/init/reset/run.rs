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

//! The reset in igc_reset_hw_base's order. Firmware (PXE, UEFI UNDI) or a
//! previous OS can leave the queues running, and a reset landing on a
//! bus-master cycle still in flight is what the master stop exists to
//! prevent, so nothing is reset until that stop is seen. The reset then has
//! to leave both queues and both engines off: Bus Master goes on after it,
//! and must not meet a queue still running.
//!
//! Queue 0 may come out of the reset enabled (Linux igc_setup_tctl and
//! igb_setup_tctl write it off for that reason). That is the part's
//! default, not a failed reset, so both queues are written off here and
//! given the clock to show it; only a queue that will not turn off, or an
//! engine the reset left on, ends the attempt.

use crate::constants::regs::{REG_RCTL, REG_RXDCTL, REG_TCTL, REG_TXDCTL};
use crate::constants::rx_bits::{RCTL_EN, RXDCTL_QUEUE_ENABLE};
use crate::constants::timeouts::QUEUE_ENABLE_MS;
use crate::constants::tx_bits::{TCTL_EN, TXDCTL_QUEUE_ENABLE};
use crate::init::wait::until;
use crate::regs::Regs;

use super::{global, master, quiesce};

pub const STILL_ON: &str = "a queue or engine still on after the reset";

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    master::stop(regs)?;
    quiesce::run(regs);
    global::run(regs)?;
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (igc_regs.h).
    unsafe {
        regs.w32(REG_RXDCTL, 0);
        regs.w32(REG_TXDCTL, 0);
    }
    // SAFETY: as above.
    let queue = || unsafe {
        regs.r32(REG_RXDCTL) & RXDCTL_QUEUE_ENABLE | regs.r32(REG_TXDCTL) & TXDCTL_QUEUE_ENABLE
    };
    let queues_off = until(QUEUE_ENABLE_MS, || queue() == 0);
    // SAFETY: as above.
    let engines = unsafe { regs.r32(REG_RCTL) & RCTL_EN | regs.r32(REG_TCTL) & TCTL_EN };
    if !queues_off || engines != 0 {
        return Err(STILL_ON);
    }
    Ok(())
}
