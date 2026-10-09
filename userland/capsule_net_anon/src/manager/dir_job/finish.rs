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

//! Turning a finished response into a body, or saying why not.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::fetch::body;
use crate::trace;

/// The inflated body of a `200` response, or `None`, traced.
pub(super) fn finish(raw: &[u8], address: [u8; 4], dir_port: u16) -> Option<Vec<u8>> {
    match body(raw) {
        Ok(out) if out.is_empty() => {
            trace::say_addr(b"dir answered with nothing", address, dir_port);
            None
        }
        Ok(out) => Some(out),
        Err(_) => {
            trace::say_addr(b"dir body not usable", address, dir_port);
            None
        }
    }
}
