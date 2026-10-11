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

//! The status poll after a SWITCH, until the card leaves programming.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::Host;
use super::super::cmds::SEND_STATUS;
use super::super::r1::{errors, switch_verdict, SwitchVerdict};
use super::status::status;

/// Poll SEND_STATUS until the card leaves programming state after a SWITCH
/// of `index`, for at most `ms`.
pub fn switch_wait<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    rca: u16,
    index: u8,
    ms: u64,
) -> EmmcResult<u32> {
    let d = Deadline::after(&h.clock, ms);
    loop {
        let s = status(h, rca)?;
        match switch_verdict(s) {
            SwitchVerdict::Done => return Ok(s),
            SwitchVerdict::Refused => return Err(EmmcError::SwitchRefused(index)),
            SwitchVerdict::Failed if errors(s) != 0 => {
                return Err(EmmcError::Status { cmd: SEND_STATUS, status: s })
            }
            SwitchVerdict::Failed => return Err(EmmcError::BadState(s)),
            SwitchVerdict::Busy => {}
        }
        if d.passed(&h.clock) {
            return Err(EmmcError::CardStuck);
        }
        h.pause(1);
    }
}
