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

//! Which of this driver's own protocol ops write the card. They belong to the
//! legacy FH load path: loading firmware sections over the FH channel (which
//! also releases the CPU), waiting for ALIVE by acknowledging interrupt
//! causes, and ringing a command doorbell on queue 4. Once the gen3 radio owns
//! the card, any of them would reset its CPU, clear the causes it polls or
//! ring a doorbell the running firmware never configured, so the serving loop
//! refuses them with `E_BUSY`. Every other op reads what setup recorded,
//! writes only the legacy staging memory, or is pure computation, and is
//! still answered.

use crate::protocol::{OP_ALIVE_WAIT, OP_FIRMWARE_LOAD, OP_HCMD_ISSUE};

/// Whether `op` writes the card's registers.
pub fn drives_card(op: u16) -> bool {
    matches!(op, OP_FIRMWARE_LOAD | OP_ALIVE_WAIT | OP_HCMD_ISSUE)
}
