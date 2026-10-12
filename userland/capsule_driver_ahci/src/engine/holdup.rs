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

//! The first wait before issue, on a port whose link is up.

use super::cause::Cause;
use super::link::ready::device_ready;
use super::port::Port;
use super::ready::{engine_runs, wait_ready};
use crate::clock::Deadline;
use crate::constants::regs::{PORT_CI, PORT_CMD, PORT_SACT, PORT_TFD};
use crate::constants::timing::READY_MS;
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Clear the errors an earlier command or link event latched (PxSERR and
/// PxIS), so only this command's are judged, and wait up to READY_MS for
/// the port to take a command: `None` when it does, else the cause to
/// recover it by. Slots the driver never issued are `Abandoned`, as after
/// issue (`Cause::of`); anything else is `Stuck`, unless the drive is still
/// in the reset a recovery sent (inside the window, the engine running, BSY
/// or DRQ set): then the request fails with `Timeout` and the drive is left
/// to finish, since another COMRESET would start its reset over.
pub(super) fn holdup(port: &Port, regs: Regs) -> AhciResult<Option<Cause>> {
    let base = port.base;
    // SAFETY: the port's register block lies inside the mapped ABAR.
    let read = |off: u32| unsafe { regs.r32(base + off) };
    super::recover::clear_errors(regs, base);
    let first = Deadline::after_ms(READY_MS);
    if wait_ready(read, || first.expired()).is_ok() {
        return Ok(None);
    }
    if read(PORT_CI) != 0 || read(PORT_SACT) != 0 {
        return Ok(Some(Cause::Abandoned));
    }
    if in_window(port) && engine_runs(read(PORT_CMD)) && !device_ready(read(PORT_TFD)) {
        return Err(AhciError::Timeout);
    }
    Ok(Some(Cause::Stuck))
}

/// Inside ATA's window after the last COMRESET a recovery sent
/// (`Port::reset_window`).
pub(super) fn in_window(port: &Port) -> bool {
    port.reset_window.is_some_and(|w| !w.expired())
}
