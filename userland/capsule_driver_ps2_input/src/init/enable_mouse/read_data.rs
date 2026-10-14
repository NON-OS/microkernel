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
use crate::init::read_port::{read_port, read_routed};
use crate::init::wait::{WaitError, REPLY_TIMEOUT_MS};

/// The mouse's reply, from the aux port only (`read_port` says why); a
/// keyboard byte that lands first goes to `keys`.
pub(super) fn read_data(grant_id: u64, keys: &mut dyn FnMut(u8)) -> Result<u8, &'static str> {
    reply(read_routed(grant_id, true, REPLY_TIMEOUT_MS, keys))
}

/// The controller's reply to a command (the configuration byte), which
/// arrives as keyboard-side data.
pub(super) fn read_config(grant_id: u64) -> Result<u8, &'static str> {
    reply(read_port(grant_id, false, REPLY_TIMEOUT_MS))
}

fn reply(read: Result<Option<u8>, WaitError>) -> Result<u8, &'static str> {
    match read {
        Ok(Some(byte)) => Ok(byte),
        Ok(None) => Err("ps2 output buffer empty"),
        Err(WaitError::Read) => Err("ps2 status read failed"),
        Err(WaitError::Timeout) => Err("ps2 output buffer empty"),
    }
}
