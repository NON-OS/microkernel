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
//! A command on whichever transport the device was bound with: BOT
//! (bot.rs) or UAS (uas.rs). Both answer the SCSI status and the residue.

use super::types::{Data, Disk};
use crate::state::State;

pub fn command(
    disk: &Disk,
    state: &mut State,
    cdb: ([u8; 16], u8),
    data: Data,
) -> Result<(u8, u32), i32> {
    if disk.uas {
        super::uas::command(disk, state, cdb, data)
    } else {
        super::bot::command(disk, state, cdb, data)
    }
}
