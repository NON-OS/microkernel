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

use crate::driver::Driver;
use crate::transaction::pace::Pace;
use crate::transaction::{TransferError, TransferRequest, TransferResult};

use super::budget::budget_ms;
use super::check_abort::check_abort;
use super::done::done;
use super::drain_rx::drain_rx;
use super::fill::fill;

/// Run one transfer on an enabled core addressed to its target: feed the
/// FIFO, drain what comes back into `out`, and end on completion, an abort
/// or the transfer's time budget, whichever comes first.
pub fn run(
    driver: &Driver,
    req: TransferRequest<'_>,
    out: &mut TransferResult,
) -> Result<(), TransferError> {
    let regs = driver.regs;
    let total = req.write.len() + req.read_len;
    let (mut wi, mut ri, mut ci) = (0usize, 0usize, 0usize);
    let mut pace = Pace::new(budget_ms(total));
    loop {
        check_abort(regs, out)?;
        let before = (ri, ci);
        drain_rx(regs, out, &mut ri, req.read_len);
        fill(driver, &req, &mut wi, &mut ci, ri);
        if ci >= total && ri >= req.read_len && done(regs) {
            /*
             * An aborted transfer empties the FIFO and idles the bus exactly
             * as a completed one does. Only the abort bit tells them apart,
             * and it can land between the check at the top of this loop and
             * here, so it is checked again before the result is trusted.
             */
            check_abort(regs, out)?;
            out.read_len = ri;
            return Ok(());
        }
        if (ri, ci) != before {
            pace.moved();
        }
        if !pace.wait() {
            return Err(TransferError::Timeout);
        }
    }
}
