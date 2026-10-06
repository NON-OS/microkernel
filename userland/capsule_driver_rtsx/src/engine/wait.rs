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

//! Waiting for TRANS_OK or TRANS_FAIL in BIPR, on the clock. What Linux's
//! ISR does on the way is done here: the bits are written back to clear
//! them, and a failed or timed-out run is stopped (rtsx_pci_stop_cmd).

use super::stop::stop;
use crate::clock::{now, Budget};
use crate::error::{Result, RtsxError};
use crate::regs::host::BIPR;
use crate::setup::Driver;
use crate::wire::{classify, Outcome};

pub fn wait(drv: &Driver, timeout_ms: u64, timeout: RtsxError, fail: RtsxError) -> Result<()> {
    let budget = Budget::begin(now(), timeout_ms);
    loop {
        let bipr = drv.regs.r32(BIPR);
        match classify(bipr) {
            Outcome::Ok => {
                drv.regs.w32(BIPR, bipr);
                return Ok(());
            }
            Outcome::Fail => {
                drv.regs.w32(BIPR, bipr);
                stop(drv);
                return Err(fail);
            }
            Outcome::Gone => return Err(RtsxError::Gone),
            Outcome::Pending => {}
        }
        if budget.spent(now()) {
            stop(drv);
            return Err(timeout);
        }
    }
}
