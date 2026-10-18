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

//! The bring-up: the backdoor's version, then the pointer turned off and
//! enabled, its identity read back, and absolute positions asked for.
//! Asking is last because the host then stops moving the PS/2 pointer.

use crate::backdoor::call;
use crate::constants::{
    CMD_ABSPOINTER_COMMAND, CMD_ABSPOINTER_DATA, CMD_ABSPOINTER_STATUS, CMD_GETVERSION, DISABLE,
    ENABLE, MAGIC, QUEUE_MASK, REQUEST_ABSOLUTE, STATUS_ERROR, VERSION_ID,
};

/// The backdoor's version, when one answers: the kernel refuses the call
/// on a host without one, and an unassigned port reads all ones.
pub fn version() -> Option<u32> {
    let r = call(CMD_GETVERSION, 0).ok()?;
    (r[1] == MAGIC && r[0] != u32::MAX).then_some(r[0])
}

pub fn enable() -> Result<(), &'static str> {
    let refused = |_| "pointer command refused";
    // Off first: enabling appends the identity to whatever is queued, so a
    // pointer left on (by firmware, or before a loss) would answer with an
    // old packet instead.
    disable();
    call(CMD_ABSPOINTER_COMMAND, ENABLE).map_err(refused)?;
    let status = call(CMD_ABSPOINTER_STATUS, 0).map_err(refused)?[0];
    if status & STATUS_ERROR == STATUS_ERROR || status & QUEUE_MASK == 0 {
        return Err("no absolute pointer");
    }
    let id = call(CMD_ABSPOINTER_DATA, 1).map_err(refused)?[0];
    if id != VERSION_ID {
        disable();
        return Err("not the absolute pointer");
    }
    call(CMD_ABSPOINTER_COMMAND, REQUEST_ABSOLUTE).map_err(refused)?;
    Ok(())
}

/// The pointer off, the host back to moving the PS/2 one.
pub fn disable() {
    let _ = call(CMD_ABSPOINTER_COMMAND, DISABLE);
}
