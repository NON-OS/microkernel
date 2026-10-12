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

//! Whether a port can take the command built in slot 0, looked at until a
//! bound the caller sets (`prepare`).

use super::link::ready::device_ready;
use crate::clock::Pace;
use crate::constants::regs::{CMD_CR, CMD_ST, PORT_CI, PORT_CMD, PORT_SACT, PORT_TFD};
use crate::error::{AhciError, AhciResult};

/// The port's command engine runs: ST and CR set (AHCI 1.3.1, 3.3.7; with
/// ST clear PxCI is never fetched).
pub(super) fn engine_runs(cmd: u32) -> bool {
    cmd & (CMD_ST | CMD_CR) == CMD_ST | CMD_CR
}

/// A port can take a command: its engine runs and its device holds neither
/// BSY nor DRQ.
fn takes_commands(cmd: u32, tfd: u32) -> bool {
    engine_runs(cmd) && device_ready(tfd)
}

/// Wait, until `expired` says the time is up, for the port to take a
/// command. `expired` is asked once after each look that finds it unable
/// to. A port that then shows any slot issued or queued holds work the
/// driver did not give it (`HbaFailed`), and nothing is issued into it.
pub fn wait_ready(
    mut read: impl FnMut(u32) -> u32,
    mut expired: impl FnMut() -> bool,
) -> AhciResult<()> {
    let pace = Pace::start();
    while !takes_commands(read(PORT_CMD), read(PORT_TFD)) {
        if expired() {
            return Err(AhciError::Timeout);
        }
        pace.rest();
    }
    if read(PORT_CI) != 0 || read(PORT_SACT) != 0 {
        return Err(AhciError::HbaFailed);
    }
    Ok(())
}
