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
use nonos_libc::Deadline;

use crate::constants::{IC_DATA_CMD, IC_DATA_CMD_STOP};
use crate::driver::Driver;
use crate::transaction::{TransferError, TransferRequest, TransferResult};

use super::budget::{budget_ms, in_flight, CLOCK_EVERY};
use super::check_abort::check_abort;
use super::done::done;
use super::drain_rx::drain_rx;
use super::read_cmd::read_cmd;
use super::rx_space::rx_space;
use super::take_write::take_write;
use super::tx_space::tx_space;

pub fn run(driver: &Driver, req: TransferRequest<'_>) -> Result<TransferResult, TransferError> {
    let regs = driver.regs;
    let mut out = TransferResult::empty();
    let total = req.write.len() + req.read_len;
    let (mut wi, mut ri, mut ci) = (0usize, 0usize, 0usize);
    let deadline = Deadline::after_ms(budget_ms(total));
    let mut spins = 0u32;
    loop {
        check_abort(regs, &mut out)?;
        drain_rx(regs, &mut out, &mut ri, req.read_len);
        while ci < total
            && tx_space(regs, driver.tx_depth) > 0
            && rx_space(regs, driver.rx_depth, in_flight(&req, ci, ri)) > 0
        {
            let mut cmd = if ci < req.write.len() {
                take_write(req.write, &mut wi)
            } else {
                read_cmd(&req, ci)
            };
            if ci == total - 1 {
                cmd |= IC_DATA_CMD_STOP;
            }
            regs.write32(IC_DATA_CMD, cmd);
            ci += 1;
        }
        if ci >= total && ri >= req.read_len && done(regs) {
            /*
             * An aborted transfer empties the FIFO and idles the bus exactly
             * as a completed one does. Only the abort bit tells them apart,
             * and it can land between the check at the top of this loop and
             * here, so it is checked again before the result is trusted.
             */
            check_abort(regs, &mut out)?;
            out.read_len = ri;
            return Ok(out);
        }
        // The clock is a system call; reading it every few rounds keeps the
        // bound in time without paying for it on every register poll.
        spins = spins.wrapping_add(1);
        if spins.is_multiple_of(CLOCK_EVERY) && deadline.expired() {
            return Err(TransferError::Timeout);
        }
        core::hint::spin_loop();
    }
}
