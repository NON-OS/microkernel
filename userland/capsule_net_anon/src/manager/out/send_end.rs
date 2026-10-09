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

//! Telling the exit a stream is finished.

use crate::cell::RELAY_END;

use super::super::state::Manager;
use super::send_relay::{send_relay, SendError};

/// Send a RELAY_END for `id` with `reason`.
///
pub fn send_end(state: &mut Manager, index: usize, id: u16, reason: u8) -> Result<(), SendError> {
    send_relay(state, index, RELAY_END, id, &[reason])
}
