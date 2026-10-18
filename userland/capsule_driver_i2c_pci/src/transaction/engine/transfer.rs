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
use crate::transaction::recover::after;
use crate::transaction::{control, valid_lengths, TransferError, TransferRequest, TransferResult};

use super::run::run;

/// One transfer from START to STOP, the bytes read landing in `out`, an
/// empty result the caller holds, whose drop wipes them
/// (types/result_wipe.rs): the engine makes no other copy of them, and none
/// is moved about by value. A request with nothing to send or receive,
/// or past the wire's limits, is refused before the bus. Every outcome is
/// counted into the bus's state, and a run of faults resets the controller
/// (recover.rs).
pub fn transfer(
    driver: &Driver,
    req: TransferRequest<'_>,
    out: &mut TransferResult,
) -> Result<(), TransferError> {
    let (w, r) = (req.write.len(), req.read_len);
    if req.addr > 0x7F || w + r == 0 || !valid_lengths(w, r) {
        return Err(TransferError::Invalid);
    }
    let result = attempt(driver, req, out);
    after(driver, &result);
    result
}

fn attempt(
    driver: &Driver,
    req: TransferRequest<'_>,
    out: &mut TransferResult,
) -> Result<(), TransferError> {
    let regs = driver.regs;
    control::wait_idle(regs)?;
    control::set_target(regs, req.addr)?;
    control::enable(regs)?;
    let result = run(driver, req, out);
    let _ = control::disable(regs);
    result
}
