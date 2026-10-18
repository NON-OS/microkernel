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

//! Feeding the transmit FIFO: the write bytes, then one read command per
//! byte wanted, the last of them carrying the STOP.

use crate::constants::{IC_DATA_CMD, IC_DATA_CMD_STOP};
use crate::driver::Driver;
use crate::transaction::TransferRequest;

use super::budget::in_flight;
use super::read_cmd::read_cmd;
use super::rx_space::rx_space;
use super::take_write::take_write;
use super::tx_space::tx_space;

/// Push commands while the TX FIFO has room and the RX FIFO can absorb
/// every read issued. `ci` counts commands pushed, `wi` write bytes taken,
/// `ri` bytes drained so far.
pub fn fill(driver: &Driver, req: &TransferRequest<'_>, wi: &mut usize, ci: &mut usize, ri: usize) {
    let regs = driver.regs;
    let total = req.write.len() + req.read_len;
    while *ci < total
        && tx_space(regs, driver.tx_depth) > 0
        && rx_space(regs, driver.rx_depth, in_flight(req, *ci, ri)) > 0
    {
        let mut cmd =
            if *ci < req.write.len() { take_write(req.write, wi) } else { read_cmd(req, *ci) };
        if *ci + 1 == total {
            cmd |= IC_DATA_CMD_STOP;
        }
        regs.write32(IC_DATA_CMD, cmd);
        *ci += 1;
    }
}
