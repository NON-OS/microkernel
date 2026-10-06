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
use crate::init::read_port;
use crate::init::wait::{WaitError, REPLY_TIMEOUT_MS};

/// The controller's reply to a command (the configuration byte), or None
/// when it gave none within the bound. A byte from the aux port is not it.
pub(super) fn read_byte(grant_id: u64) -> Result<Option<u8>, &'static str> {
    match read_port(grant_id, false, REPLY_TIMEOUT_MS) {
        Ok(byte) => Ok(byte),
        Err(WaitError::Read) => Err("kbd status read failed"),
        Err(WaitError::Timeout) => Ok(None),
    }
}
