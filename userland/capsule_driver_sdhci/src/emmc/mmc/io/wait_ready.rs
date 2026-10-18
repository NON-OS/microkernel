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

//! The status poll after a write, until the card is ready for data.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::Host;
use super::super::ops::status;
use super::super::r1::{errors, ready};

/// Poll SEND_STATUS until the card is ready for data in transfer state. An
/// error bit in the status (a write the card could not program) fails.
pub fn wait_ready<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    rca: u16,
    ms: u64,
) -> EmmcResult<()> {
    let d = Deadline::after(&h.clock, ms);
    loop {
        let s = status(h, rca)?;
        if errors(s) != 0 {
            return Err(EmmcError::Status { cmd: super::super::cmds::SEND_STATUS, status: s });
        }
        if ready(s) {
            return Ok(());
        }
        if d.passed(&h.clock) {
            return Err(EmmcError::CardStuck);
        }
        h.pause(1);
    }
}
