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

use super::conds::{mcu_rxtx_empty, mitigate_rxtx_empty, tx_fifo_empty};
use super::{request_stop, wait_for};
use crate::chip::MacVersion;
use crate::regs::Regs;

/// Linux waits 42 polls 100 us apart for each FIFO condition: 4.2 ms.
const FIFO_MS: u64 = 5;

fn met(done: bool, what: &'static str) -> Result<(), &'static str> {
    done.then_some(()).ok_or(what)
}

/// Linux rtl_wait_txrx_fifo_empty. A FIFO that does not drain is answered
/// with the condition that did not come true; the caller logs it and goes
/// on as Linux does, since a reset or a start follows either way.
pub fn wait_txrx_fifo_empty(regs: &Regs, ver: MacVersion) -> Result<(), &'static str> {
    let mcu = || met(wait_for(FIFO_MS, true, || mcu_rxtx_empty(regs)), "MCU RXTX_EMPTY");
    match ver.0 {
        40..=52 => {
            met(wait_for(FIFO_MS, true, || tx_fifo_empty(regs)), "TxConfig TXCFG_EMPTY")?;
            mcu()
        }
        61 => mcu(),
        63.. => {
            request_stop(regs);
            mcu()?;
            met(wait_for(FIFO_MS, true, || mitigate_rxtx_empty(regs)), "IntrMitigate 0x0103")
        }
        _ => Ok(()),
    }
}
