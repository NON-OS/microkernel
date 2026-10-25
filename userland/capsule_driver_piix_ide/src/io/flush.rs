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

//! FLUSH CACHE (EXT): a non-data command, done when the disk lets go of
//! the bus.

use super::status::E_IO;
use super::taskfile::Taskfile;
use super::transfer::recover;
use crate::channel::Channel;
use crate::constants::ata::{ST_DF, ST_ERR};
use crate::constants::timing::COMMAND_MS;

pub fn flush(ch: &Channel, drive: u8, tf: &Taskfile) -> Result<(), i32> {
    if !ch.ready(drive) {
        return Err(recover(ch));
    }
    tf.issue(&ch.cmd);
    if !ch.not_busy(COMMAND_MS) {
        return Err(recover(ch));
    }
    if ch.status() & (ST_ERR | ST_DF) != 0 {
        return Err(E_IO);
    }
    Ok(())
}
