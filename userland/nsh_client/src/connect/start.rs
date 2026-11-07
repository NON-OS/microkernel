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

//! What goes out once the host has let the user in.

use alloc::vec::Vec;
use core::time::Duration;

/// What goes out once the host has let the user in: the terminal's size,
/// and each escalation grant in turn, the next once the last is answered.
pub struct Start {
    pub window: Option<(u16, u16)>,
    pub escalate: Vec<Vec<u8>>,
    /// How long the host has, from the connection, to admit the login.
    pub handshake: Duration,
    /// How long the host has, once the session is closed, to end it.
    pub close_wait: Duration,
}
