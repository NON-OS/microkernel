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

//! `read` on a socket: a receive into one buffer, with no flags.

use alloc::vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::inbound::recv;

/// `read` on a socket.
pub fn read(guest: &Guest, id: u32, buf: u64, len: u64) -> u64 {
    match recv(guest, id, &vec![(buf, len)], 0, 0) {
        Ok(got) => errno::ok(got.n as u64),
        Err(e) => e,
    }
}
