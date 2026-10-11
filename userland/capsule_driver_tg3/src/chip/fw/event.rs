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

//! Commands to the firmware: words in the command mailbox, then the driver
//! event bit, once the previous event is acknowledged.

use crate::chip::sram;
use crate::constants::grc::{GRC_RX_CPU_DRIVER_EVENT, GRC_RX_CPU_EVENT};
use crate::constants::sram::*;
use crate::constants::timeouts::FW_EVENT_MS;
use crate::init::wait;
use crate::regs::Regs;

/*
 * SAFETY for the register accesses: `regs` carries the broker MmioMap base
 * for BAR0 and GRC_RX_CPU_EVENT is a 32-bit register inside it.
 */
fn event(regs: &Regs) {
    unsafe { regs.w32f(GRC_RX_CPU_EVENT, regs.r32(GRC_RX_CPU_EVENT) | GRC_RX_CPU_DRIVER_EVENT) };
}

/// tg3_wait_for_event_ack. A firmware that never acknowledges is not an
/// error: Linux goes on after the same wait.
fn wait_ack(regs: &Regs) {
    let _ = wait::until(FW_EVENT_MS, || unsafe {
        regs.r32(GRC_RX_CPU_EVENT) & GRC_RX_CPU_DRIVER_EVENT == 0
    });
}

fn command(regs: &Regs, words: &[(u32, u32)]) {
    wait_ack(regs);
    for &(at, v) in words {
        sram::write(regs, at, v);
    }
    event(regs);
}

pub fn stop(regs: &Regs, asf: bool) {
    if asf {
        command(regs, &[(NIC_SRAM_FW_CMD_MBOX, FWCMD_NICDRV_PAUSE_FW)]);
        wait_ack(regs);
    }
}

pub fn heartbeat(regs: &Regs, asf: bool) {
    if asf {
        command(
            regs,
            &[
                (NIC_SRAM_FW_CMD_MBOX, FWCMD_NICDRV_ALIVE3),
                (NIC_SRAM_FW_CMD_LEN_MBOX, 4),
                (NIC_SRAM_FW_CMD_DATA_MBOX, TG3_FW_UPDATE_TIMEOUT_SEC),
            ],
        );
    }
}
