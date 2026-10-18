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

//! Waiting on the one command the driver issues, in slot 0, judged from the
//! port registers alone. `read` returns the port register at an offset from
//! the port base, so the host proofs run these same loops against a model of
//! a device.

use super::verdict::{verdict, Verdict};
use crate::clock::Pace;
use crate::constants::regs::{PORT_CI, PORT_IS, PORT_SACT, PORT_TFD};
use crate::error::{AhciError, AhciResult};

/// Wait, until `expired` says the time is up, for the command in slot 0 to
/// finish. `expired` is asked once after each round that leaves it pending.
/// Each round reads PxCI first, then PxSACT, PxIS and PxTFD, so a clear slot
/// bit is judged against a status read after it, one the HBA posted before
/// it cleared the bit. Read the other way round, a command that failed
/// between the status read and the PxCI read would pass as done.
pub fn wait_done(
    mut read: impl FnMut(u32) -> u32,
    mut expired: impl FnMut() -> bool,
) -> AhciResult<()> {
    let pace = Pace::start();
    loop {
        let ci = read(PORT_CI);
        let sact = read(PORT_SACT);
        let is = read(PORT_IS);
        let tfd = read(PORT_TFD);
        match verdict(ci, sact, is, tfd) {
            Verdict::Done => return Ok(()),
            Verdict::Failed => return Err(AhciError::CommandFailed),
            Verdict::Pending => {}
        }
        if expired() {
            return Err(AhciError::Timeout);
        }
        pace.rest();
    }
}
