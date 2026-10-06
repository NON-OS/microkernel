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
use crate::constants::{KBD_BAT_OK, KBD_RESET, MOUSE_ACK};
use crate::init::enable_scanning::{read_reply, send};
use crate::init::wait::{ACK_TIMEOUT_MS, BAT_TIMEOUT_MS};

/// Reset the keyboard: 0xFF, its ACK (0xFA), then the self-test result
/// (0xAA) once the basic assurance test is done, which takes a real
/// keyboard hundreds of milliseconds. True when the keyboard passed.
pub(super) fn reset(grant_id: u64) -> Result<bool, &'static str> {
    if send(grant_id, KBD_RESET, ACK_TIMEOUT_MS)? != Some(MOUSE_ACK) {
        return Ok(false);
    }
    Ok(read_reply(grant_id, BAT_TIMEOUT_MS)? == Some(KBD_BAT_OK))
}
