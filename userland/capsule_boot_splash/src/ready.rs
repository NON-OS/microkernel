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

//! Finding the compositor: the splash starts beside it, so it asks for the
//! name and a healthcheck a bounded number of times before giving up.

use nonos_libc::mk_yield;

use crate::proto;

const READY_ATTEMPTS: u32 = 256;

pub(crate) fn wait_compositor() -> Option<u32> {
    for _ in 0..READY_ATTEMPTS {
        if let Some(p) = proto::lookup(b"compositor") {
            if proto::healthcheck(p, 1).is_ok() {
                return Some(p);
            }
        }
        mk_yield();
    }
    None
}
