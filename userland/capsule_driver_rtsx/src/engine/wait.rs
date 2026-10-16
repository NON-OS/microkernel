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

//! Waiting for TRANS_OK or TRANS_FAIL in BIPR, on the clock, parked between
//! polls once past the first millisecond (C7). What Linux's ISR does on the
//! way is done here: the bits are written back to clear them, and a run
//! that failed, timed out, lost its card or found the reader gone is
//! stopped (rtsx_pci_stop_cmd), so no engine is left running against the
//! buffers (C1). A card-detect change is left latched for the slot's
//! watcher (serve/slot.rs) to see.

use super::stop::stop;
use crate::clock::{now, pause, Budget};
use crate::error::{Result, RtsxError};
use crate::regs::host::BIPR;
use crate::setup::Driver;
use crate::wire::{classify, Outcome};

pub fn wait(drv: &Driver, timeout_ms: u64, timeout: RtsxError, fail: RtsxError) -> Result<()> {
    let budget = Budget::begin(now(), timeout_ms);
    loop {
        let bipr = drv.regs.r32(BIPR);
        let ended = match classify(bipr, drv.card_on) {
            Outcome::Pending => None,
            Outcome::Ok => {
                drv.regs.w32(BIPR, bipr);
                return Ok(());
            }
            Outcome::Fail => {
                drv.regs.w32(BIPR, bipr);
                Some(fail)
            }
            Outcome::Gone => Some(RtsxError::Gone),
            Outcome::Removed => Some(RtsxError::Removed),
        };
        let t = now();
        let ended = ended.or(budget.spent(t).then_some(timeout));
        if let Some(e) = ended {
            stop(drv);
            return Err(e);
        }
        pause(&budget, t);
    }
}
